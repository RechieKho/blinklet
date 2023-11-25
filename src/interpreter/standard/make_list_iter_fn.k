return
    closure
        parameter ls
        var length
            list-length ls
        var index 0
        return
            closure
                when
                    ge index length
                    return null
                var element
                    list-get ls index
                set index
                    add index 1
                return element
