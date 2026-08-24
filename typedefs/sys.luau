export type sys = {
    hostName: () -> string,
    name: () -> string,
    memory: () -> MemoryInfo,
    upTime: () -> number
}

local sys = {}

function sys.hostName(): string
    return ""
end

function sys.name(): string
    return ""
end

export type MemoryInfo = {
    total: number,
    free: number
}

function sys.memory(): MemoryInfo
    return {} :: MemoryInfo
end

function sys.upTime(): number
    return 0
end

return sys :: sys