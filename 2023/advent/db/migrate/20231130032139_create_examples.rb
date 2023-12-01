class CreateExamples < ActiveRecord::Migration[7.1]
  def change
    create_table :examples do |t|
      t.references :day
      t.text :input
      t.text :expected_output

      t.timestamps
    end
  end
end
