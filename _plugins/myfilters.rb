module MyFilters

    def file_date(input)
      File.mtime(input)
    end

	Liquid::Template.register_filter self
end

