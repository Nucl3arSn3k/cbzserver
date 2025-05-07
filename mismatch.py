import magic
import os

import re
class FileStatus:
    filepath:str
    err_file: bool


def whitespace_checker(file_path):
    stat = FileStatus()
    pattern = re.compile(r'.*\s+\.(cbr|cbz)$', re.IGNORECASE)
    stat.filepath = file_path
    if pattern.match(file_path):
        stat.err_file = True
    else:
        stat.err_file = False

    return stat





def first_bytes(file_path, num_bytes=8):
    with open(file_path, 'rb') as input:
        return list(input.read(num_bytes))


def recursive_search(start_location):
    classhold = []
    entries = os.listdir(start_location)
    for x in entries:
        f_p = os.path.join(start_location, x)
        is_dir = os.path.isdir(f_p)
        
        if is_dir:
            classhold.extend(recursive_search(f_p))
        else:
            print(f_p)
            
            if f_p.endswith(('.cbr', '.cbz')):
                stat = whitespace_checker(f_p)
                classhold.append(stat)
    
    return classhold
                




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
                fp = val.filepath
                new_filepath = re.sub(r'(\s+)(\.\w+$)', r'\2', fp)
                try:
                    os.rename(fp, new_filepath)
                    print(f"Renamed: {fp} -> {new_filepath}")
                    file.write(new_filepath + '\n') 
                except Exception as e:
                    print(f"Error renaming {fp}: {e}")
            
    


if __name__ == "__main__":
    main()