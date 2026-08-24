export type fmt = {
    bytes: (bytes: number) -> string,
    time: (seconds: number) -> string
}

local fmt = {}

function fmt.bytes(bytes: number): string
    return ""
end

function fmt.time(seconds: number): string
    return ""
end

return fmt :: fmt