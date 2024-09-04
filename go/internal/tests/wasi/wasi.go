//go:generate go run github.com/ydnar/wasm-tools-go/cmd/wit-bindgen-go@v0.1.5 generate -w service -o bindings ./wit

package wasi

import (
	"log/slog"
	"unsafe"

	incominghandler "github.com/rvolosatovs/west/go/internal/tests/wasi/bindings/wasi/http/incoming-handler"
	"github.com/rvolosatovs/west/go/internal/tests/wasi/bindings/wasi/http/types"
	"github.com/ydnar/wasm-tools-go/cm"
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
	res := types.NewOutgoingResponse(req.Headers())

	slog.Debug("getting response body")
	body := res.Body()
	if body.IsErr() {
		slog.Debug("failed to get body")
		return ptr(types.ErrorCodeInternalError(cm.Some("failed to get body")))
	}
	bodyOut := body.OK()

	slog.Debug("getting response body stream")
	bodyWrite := bodyOut.Write()
	if bodyWrite.IsErr() {
		slog.Debug("failed to get response body stream")
		return ptr(types.ErrorCodeInternalError(cm.Some("failed to get response body stream")))
	}

	slog.Debug("setting response outparam")
	types.ResponseOutparamSet(out, cm.OK[cm.Result[types.ErrorCodeShape, types.OutgoingResponse, types.ErrorCode]](res))
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
