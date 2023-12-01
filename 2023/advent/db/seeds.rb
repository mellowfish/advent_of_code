(1..25).each do |day_number|
  day = Day.find_or_create_by(id: day_number)
  day.update(title: "Day #{day_number}") if day.title.blank?
end
