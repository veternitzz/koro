local fs = require("@self/fs")
local io = require("@self/io")
local sys = require("@self/sys")
local fmt = require("@self/fmt")

export type std = {
    fs: fs.Fs,
    io: io.Io,
    sys: sys.sys,
    fmt: fmt.fmt
}

return {
    fs = fs :: fs.Fs,
    io = io :: io.Io,
    sys = sys :: sys.sys,
    fmt = fmt :: fmt.fmt
} :: std