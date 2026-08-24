export type Io = {
    write: (string) -> (),
    ewrite: (string) -> (),
    readLine: () -> string
}

local io = {}

function io.write(s: string): ()
    return
end

function io.ewrite(s: string): ()
    return
end

function io.readLine(): string
    return ""
end

return io :: Io