#!/opt/homebrew/bin/fish

function build_debug
  tailwindcss -i tailwind/tailwind.css -o tailwind/out/tailwind.css
  trunk build 
end

function build_release
  tailwindcss -i tailwind/tailwind.css -o tailwind/out/tailwind.css --minify
  trunk build --release
end

function serve_debug
  build_debug
  cd ../server
  cargo run
end

function serve_release
  build_release
  cd ../server
  cargo run --release
end

function help
  echo "-sd --serve_debug"
  echo "-sr --serve_release"
  echo "-d --build_debug"
  echo "-r --build_release"
end

if test -z $argv
  serve_debug
end

for arg in $argv
  switch $arg
    case -sd --serve_debug
      serve_debug
    case -sr --serve_release
      serve_release
    case -d --debug
      build_debug
    case -r --release
      build_release
    case -h --help; or '*'
      help
  end
end



