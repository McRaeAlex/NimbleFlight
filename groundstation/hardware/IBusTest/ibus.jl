import JSON

struct IBusPacket do 
    header::Int16
    channels::Vector{UInt16, 14}
    checksum::Int16
end

function from_bytes(bytes) do IBusPacket

end

function convert_raw_to_json() do 
    open("./ibus_serial_out.hex") do raw
        open("./ibus_serial_out.json") do parsed

        end
    end
end