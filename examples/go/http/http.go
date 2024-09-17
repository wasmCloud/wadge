//go:generate go run github.com/bytecodealliance/wasm-tools-go/cmd/wit-bindgen-go generate -w app -o bindings ./wit

package wasi

import (
	"log/slog"
	"unsafe"

	"github.com/bytecodealliance/wasm-tools-go/cm"
	incominghandler "github.com/rvolosatovs/west/tests/go/wasi/bindings/wasi/http/incoming-handler"
	"github.com/rvolosatovs/west/tests/go/wasi/bindings/wasi/http/types"
)

func init() {
	incominghandler.Exports.Handle = func(request types.IncomingRequest, responseOut types.ResponseOutparam) {
		if err := handle(request, responseOut); err != nil {
			types.ResponseOutparamSet(responseOut, cm.Err[cm.Result[types.ErrorCodeShape, types.OutgoingResponse, types.ErrorCode]](*err))
		}
	}
}

func ptr[T any](v T) *T {
	return &v
}

func handle(req types.IncomingRequest, out types.ResponseOutparam) *types.ErrorCode {
	slog.Debug("constructing new response")
	resp := types.NewOutgoingResponse(req.Headers())

	slog.Debug("getting response body")
	body := resp.Body()
	if body.IsErr() {
		slog.Debug("failed to get response body")
		return ptr(types.ErrorCodeInternalError(cm.Some("failed to get response body")))
	}
	bodyOut := body.OK()

	slog.Debug("getting response body stream")
	bodyWrite := bodyOut.Write()
	if bodyWrite.IsErr() {
		slog.Debug("failed to get response body stream")
		return ptr(types.ErrorCodeInternalError(cm.Some("failed to get response body stream")))
	}

	slog.Debug("setting response outparam")
	types.ResponseOutparamSet(out, cm.OK[cm.Result[types.ErrorCodeShape, types.OutgoingResponse, types.ErrorCode]](resp))
	stream := bodyWrite.OK()
	s := "foo bar baz"
	writeRes := stream.BlockingWriteAndFlush(cm.NewList(unsafe.StringData(s), uint(len(s))))
	if writeRes.IsErr() {
		slog.Error("failed to write to stream", "err", writeRes.Err())
		return nil
	}
	slog.Debug("dropping body stream")
	stream.ResourceDrop()

	slog.Debug("finishing outgoing body")
	finishRes := types.OutgoingBodyFinish(*bodyOut, cm.None[types.Fields]())
	if finishRes.IsErr() {
		slog.Error("failed to finish outgoing body", "err", finishRes.Err())
		return nil
	}
	return nil
}
