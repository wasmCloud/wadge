// Code generated by wit-bindgen-go. DO NOT EDIT.

// Package logging represents the imported interface "wasi:logging/logging@0.1.0-draft".
//
// WASI Logging is a logging API intended to let users emit log messages with
// simple priority levels and context values.
package logging

import (
	"go.bytecodealliance.org/cm"
)

// Level represents the enum "wasi:logging/logging@0.1.0-draft#level".
//
// A log level, describing a kind of message.
//
//	enum level {
//		trace,
//		debug,
//		info,
//		warn,
//		error,
//		critical
//	}
type Level uint8

const (
	// Describes messages about the values of variables and the flow of
	// control within a program.
	LevelTrace Level = iota

	// Describes messages likely to be of interest to someone debugging a
	// program.
	LevelDebug

	// Describes messages likely to be of interest to someone monitoring a
	// program.
	LevelInfo

	// Describes messages indicating hazardous situations.
	LevelWarn

	// Describes messages indicating serious errors.
	LevelError

	// Describes messages indicating fatal errors.
	LevelCritical
)

var _LevelStrings = [6]string{
	"trace",
	"debug",
	"info",
	"warn",
	"error",
	"critical",
}

// String implements [fmt.Stringer], returning the enum case name of e.
func (e Level) String() string {
	return _LevelStrings[e]
}

// MarshalText implements [encoding.TextMarshaler].
func (e Level) MarshalText() ([]byte, error) {
	return []byte(e.String()), nil
}

// UnmarshalText implements [encoding.TextUnmarshaler], unmarshaling into an enum
// case. Returns an error if the supplied text is not one of the enum cases.
func (e *Level) UnmarshalText(text []byte) error {
	return _LevelUnmarshalCase(e, text)
}

var _LevelUnmarshalCase = cm.CaseUnmarshaler[Level](_LevelStrings[:])

// Log represents the imported function "log".
//
// Emit a log message.
//
// A log message has a `level` describing what kind of message is being
// sent, a context, which is an uninterpreted string meant to help
// consumers group similar messages, and a string containing the message
// text.
//
//	log: func(level: level, context: string, message: string)
//
//go:nosplit
func Log(level Level, context string, message string) {
	level0 := (uint32)(level)
	context0, context1 := cm.LowerString(context)
	message0, message1 := cm.LowerString(message)
	wasmimport_Log((uint32)(level0), (*uint8)(context0), (uint32)(context1), (*uint8)(message0), (uint32)(message1))
	return
}
