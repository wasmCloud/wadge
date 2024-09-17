//go:generate go run github.com/rvolosatovs/west/cmd/west-bindgen-go

package wasi_test

import (
	"testing"
	"unsafe"

	"github.com/bytecodealliance/wasm-tools-go/cm"
	west "github.com/rvolosatovs/west"
	_ "github.com/rvolosatovs/west/bindings"
	testtypes "github.com/rvolosatovs/west/bindings/wasi/http/types"
	httpext "github.com/rvolosatovs/west/bindings/wasiext/http/ext"
	incominghandler "github.com/rvolosatovs/west/tests/go/wasi/bindings/wasi/http/incoming-handler"
	"github.com/rvolosatovs/west/tests/go/wasi/bindings/wasi/http/types"
	"github.com/stretchr/testify/assert"
)

func TestIncomingHandler(t *testing.T) {
	west.RunTest(t, func() {
		headers := testtypes.NewFields()
		headers.Append(
			testtypes.FieldKey("foo"),
			testtypes.FieldValue(cm.NewList(
				unsafe.SliceData([]byte("bar")),
				3,
			)),
		)
		headers.Append(
			testtypes.FieldKey("foo"),
			testtypes.FieldValue(cm.NewList(
				unsafe.SliceData([]byte("baz")),
				3,
			)),
		)
		headers.Set(
			testtypes.FieldKey("key"),
			cm.NewList(
				unsafe.SliceData(
					[]testtypes.FieldValue{
						testtypes.FieldValue(cm.NewList(
							unsafe.SliceData([]byte("value")),
							5,
						)),
					},
				),
				1,
			),
		)
		req := testtypes.NewOutgoingRequest(headers)
		req.SetPathWithQuery(cm.Some("test"))
		req.SetMethod(testtypes.MethodGet())
		out := httpext.NewResponseOutparam()
		incominghandler.Exports.Handle(
			types.IncomingRequest(httpext.NewIncomingRequest(req)),
			types.ResponseOutparam(out.F0),
		)
		out.F1.Subscribe().Block()
		respOptResRes := out.F1.Get()
		respResRes := respOptResRes.Some()
		if !assert.NotNil(t, respResRes) {
			t.FailNow()
		}
		respRes := respResRes.OK()
		if !assert.NotNil(t, respRes) || !assert.Nil(t, respRes.Err()) {
			t.FailNow()
		}
		resp := respRes.OK()
		assert.Equal(t, testtypes.StatusCode(200), resp.Status())
		hs := map[string][][]byte{}
		for _, h := range resp.Headers().Entries().Slice() {
			k := string(h.F0)
			hs[k] = append(hs[k], h.F1.Slice())
		}
		assert.Equal(t, map[string][][]byte{
			"foo": {
				[]byte("bar"),
				[]byte("baz"),
			},
			"key": {
				[]byte("value"),
			},
		}, hs)
		bodyRes := resp.Consume()
		body := bodyRes.OK()
		if !assert.NotNil(t, body) {
			t.FailNow()
		}
		bodyStreamRes := body.Stream()
		bodyStream := bodyStreamRes.OK()
		if !assert.NotNil(t, bodyStream) {
			t.FailNow()
		}
		bufRes := bodyStream.BlockingRead(4096)
		buf := bufRes.OK()
		if !assert.NotNil(t, buf) {
			t.FailNow()
		}
		assert.Equal(t, []byte("foo bar baz"), buf.Slice())
		bodyStream.ResourceDrop()
	})
}
