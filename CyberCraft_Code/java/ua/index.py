import os

def remove_copy_files(folder_path):
    for root, dirs, files in os.walk(folder_path):
        for file_name in files:
            if file_name.endswith('_copy.html'):
                os.remove(os.path.join(root, file_name))

# Викликаємо функцію для видалення файлів з префіксом '_copy' з вказаної папки
remove_copy_files('C:/Users/Ukraine/AndroidStudioProjects/CCSCode/app/src/main/assets/html/java/ua/')
