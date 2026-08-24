export type Fs = {
    read: (kind: "d" | "f", path: string) -> (string | { string }),
    exists: (path: string) -> (boolean),
    writeFile: (path: string, contents: string) -> (),
    writeDir: (path: string) -> (),
    writeDirAll: (path: string) -> (),
    removeFile: (path: string) -> (),
    removeDir: (path: string) -> (),
    removeDirAll: (path: string) -> (),
    metadata: (path: string) -> FileMetadata
}

local fs = {}

function fs.read(kind: "d" | "f", path: string): string | { string }
    return "", {""}
end

function fs.exists(path: string): boolean
    return true, false
end

function fs.writeFile(path: string, contents: string): ()
    return
end

function fs.writeDir(path: string): ()
    return
end

function fs.writeDirAll(path: string): ()
    return
end

function fs.removeFile(path: string): ()
    return
end

function fs.removeDir(path: string): ()
    return
end

function fs.removeDirAll(path: string): ()
    return
end

export type FileMetadata = {
    size: (self: FileMetadata) -> number
}

function fs.metadata(path: string): FileMetadata
    local fm: FileMetadata = nil
    
    return fm :: FileMetadata
end

return fs :: Fs