import magic
import os


class FileStatus:
    filepath:str
    err_file: bool


def first_bytes(file_path, num_bytes=8):
    with open(file_path, 'rb') as input:
        return list(input.read(num_bytes))


def recursive_search(start_location):
    classhold = []
    entries = os.listdir(start_location)
    for x in entries:
        l_fstat = FileStatus()
        f_p = os.path.join(start_location,x)
        is_dir = os.path.isdir(f_p)
        if is_dir:
            #v2al = recursive_search(f_p)
            classhold.extend(recursive_search(f_p))
        else:
            l_fstat.filepath = f_p
            val3 = first_bytes(f_p)
            
            #val2 = magic_search(f_p)
            print(f_p)
            # ZIP signature: 50 4B 03 04
            ZIP_SIG = [0x50, 0x4B, 0x03, 0x04]
            # RAR signatures
            RAR4_SIG = [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00]
            RAR5_SIG = [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00]

            if f_p.endswith(('.cbr', '.cbz')):
                if val3[:4] == ZIP_SIG and f_p.endswith('.cbr'):
                    l_fstat.err_file = True
                elif (val3[:7] == RAR4_SIG or val3 == RAR5_SIG) and f_p.endswith('.cbz'):
                    l_fstat.err_file = True
                else:
                    l_fstat.err_file = False



    return classhold


def magic_search(file_path):
    mime = magic.Magic(mime=True)
    file_type = mime.from_file(file_path)
    
    return file_type


#this is a little script to adress potential file mismatches in my comic folder. Including it because some of these archives have mismatches
def main():

    #searchstr = "I:\\Comics\\2000AD (0000-2162+)(1977-)\\2000AD 0742 (1991) (Gigman).cbr"
    #val2 = magic_search(searchstr)
    #stl = len(searchstr)
    #print(searchstr[stl -3:stl])
    """
    if val2 == "application/zip" and searchstr[stl -3:stl]:
        print("Mismatch")
    """
    x = recursive_search("I:\\Comics")
    with open("logfile.txt", 'w', encoding='utf-8') as file:
        for val in x:
            if val.err_file == True:
                file.write(val.filepath + '\n')  # Also added newline

    #val3  = magic_search("I:\\Comics\\2000AD (0000-2162+)(1977-)\\2000AD 0743 (1991) (jaseb).cbr")


if __name__ == "__main__":
    main()