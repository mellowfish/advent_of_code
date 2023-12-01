class CreateDays < ActiveRecord::Migration[7.1]
  def change
    create_table :days do |t|
      t.string :title
      t.text :description
      t.text :solution

      t.timestamps
    end
  end
end
