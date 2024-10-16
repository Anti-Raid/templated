@pragma {"lang":"lua","a":1}
function (args) 
    --[[
        templated 0.1.0
    ]]--
    if 1==1 then
        local a

a = {
    cache = {},
    load = function(
        b
    )
        if not a.cache[b] then
            a.cache[b] = {
                c = a[b](),
            }
        end

        return a.cache[b].c
    end,
}

do
    function a.a(
    )
        function double(
            b
        )
            return b * 2
        end

        return {double = double}
    end
end

function main(
    b
)
    local c = a.load'a'
    local d = 1

    print(c.double(d))
end

main()

        -- Return main
        return main(args)
    end
end