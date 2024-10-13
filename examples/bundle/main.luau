function main(args)
    local double = require("./double.luau")

    local a = 1
    for i, v in ipairs(args) do
        a += 1
    end

    function abc() 
        return 1 -- Non root level return is OK
    end -- This should not be symbol mangled

    if 1 == 1 -- Non root level if
    then
        return 1
    end

    return double.double(a)
end