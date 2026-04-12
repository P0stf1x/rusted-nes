with open("Composite_wiki.pal", "rb") as f:
    print("[", end="\n\t")
    bytes = f.read1(192)
    for i in range(64):
        color_triplet = bytes[:3]
        bytes = bytes[3:]
        red, green, blue = color_triplet
        end = ", "
        if i % 16 == 15:
            end = ",\n\t"
        if i == 63:
            end = "\n"
        print(f"0xFF{red:02X}{green:02X}{blue:02X}", end=end)
    print("]")
