abstract type LibraryItem end

mutable struct Book <: LibraryItem
    name::String
    author::String
    genre::String
    available::Bool
end

mutable struct Film <: LibraryItem
    name::String
    genre::String
    available::Bool
end

mutable struct Article <: LibraryItem
    name::String
    author::String
    related::String
    available::Bool
end

mutable struct Library
    items::Vector{LibraryItem}
end

function display_info(item::Book)
    println("$(item.name) by $(item.author)")
    println("$(item.genre)")
    if item.available == false 
        println("Not Available")
    end
end

function display_info(item::Film)
    println("$(item.name)")
    println("$(item.genre)")
    if item.available == false 
        println("Not Available")
    end
end

function display_info(item::Article)
    println("$(item.name)")
    println("$(item.author)")
    println("about $(item.related)")
    if item.available == false 
        println("Not Available")
    end
end

function add_item(library::Library, item::LibraryItem)
    push!(library.items, item)
end

function display_all(library::Library)
    for item in library.items
        display_info(item)
        println()
    end
end

library = Library(LibraryItem[])
book1 = Book("Hitchhiker's Guide to the Galaxy", "Douglas Adams", "sci-fi", true)
film1 = Film("Fight Club", "Thriller", false)

add_item(library, book1)
add_item(library, film1)

display_all(library)

