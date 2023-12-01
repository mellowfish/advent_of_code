class SeparateOutParts < ActiveRecord::Migration[7.1]
  def up
    change_table :days do |t|
      t.rename :solution, :solution_part_one
      t.text :solution_part_two
    end

    add_column :examples, :part, :string
  end

  def down
    raise ActiveRecord::IrreversibleMigration
  end
end
