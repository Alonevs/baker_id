Add-Type -AssemblyName System.IO.Compression.FileSystem
Add-Type -AssemblyName System.Web

$docxFiles = Get-ChildItem -Path "c:\Users\xico0\.gemini\antigravity\scratch\rvc_sprite_baker_poc\doc\*.docx"
foreach ($file in $docxFiles) {
    Write-Host "Procesando $($file.Name)..."
    $zipPath = $file.FullName
    try {
        $archive = [System.IO.Compression.ZipFile]::OpenRead($zipPath)
        $entry = $archive.Entries | Where-Object { $_.FullName -eq "word/document.xml" }
        if ($entry) {
            $stream = $entry.Open()
            $reader = New-Object System.IO.StreamReader($stream)
            $xmlText = $reader.ReadToEnd()
            $reader.Close()
            $stream.Close()
            
            # Extraer contenido entre etiquetas <w:t>
            # Para manejar XML más complejo, eliminamos etiquetas pero conservamos texto
            # Reemplazar etiquetas con espacios o saltos de línea para mantener estructura básica
            $cleanText = $xmlText
            
            # Insertar saltos de línea en párrafos
            $cleanText = $cleanText -replace '<w:p\b[^>]*>', "`n"
            $cleanText = $cleanText -replace '<w:br\b[^>]*>', "`n"
            $cleanText = $cleanText -replace '<w:tab\b[^>]*>', "`t"
            
            # Filtrar todas las etiquetas XML excepto el texto dentro de <w:t>
            # Buscamos coincidencias de <w:t>...</w:t>
            $matches = [regex]::matches($cleanText, '<w:t[^>]*>(.*?)</w:t>')
            $paragraphs = @()
            
            # Una forma simple es usar un parser XML si es posible, pero regex es más tolerante a namespaces no declarados
            $textBuilder = New-Object System.Text.StringBuilder
            foreach ($match in $matches) {
                $val = $match.Groups[1].Value
                # Decodificar entidades XML básicas
                $val = $val -replace '&amp;', '&'
                $val = $val -replace '&lt;', '<'
                $val = $val -replace '&gt;', '>'
                $val = $val -replace '&quot;', '"'
                $val = $val -replace '&apos;', "'"
                [void]$textBuilder.Append($val)
            }
            
            # Alternativa: extraer texto plano usando expresión regular eliminando todo lo que está entre < y >
            # Pero para w:t es más seguro. Vamos a intentar con un método de reemplazo XML simple
            # ya que a veces w:t está anidado y el regex simple funciona bien.
            
            $text = $textBuilder.ToString()
            # Si el texto está vacío, intentamos otra regex más genérica
            if ([string]::IsNullOrEmpty($text)) {
                # Eliminar etiquetas
                $text = $xmlText -replace '<[^>]+>', ' '
                $text = $text -replace '\s+', ' '
            }
            
            $outPath = Join-Path $file.DirectoryName ($file.BaseName + ".txt")
            $text | Out-File -FilePath $outPath -Encoding utf8
            Write-Host "Guardado en $outPath"
        } else {
            Write-Warning "No se encontró word/document.xml en $($file.Name)"
        }
        $archive.Dispose()
    } catch {
        Write-Error "Error al procesar $($file.Name): $_"
    }
}
