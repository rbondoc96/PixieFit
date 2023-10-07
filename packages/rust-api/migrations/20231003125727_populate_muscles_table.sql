INSERT INTO muscles (group_id, name, simple_name, image_source) VALUES
    ((SELECT id FROM muscle_groups WHERE name = 'Arms'), 'Biceps Brachii', 'Biceps', 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Arms'), 'Triceps Brachii', 'Triceps', 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Calves'), 'Calves', NULL, 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Chest'), 'Pectoralis', 'Pecs', 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Core'), 'Abdominals', 'Abs', 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Forearms'), 'Forearms', NULL, 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Glutes'), 'Glutes', NULL, 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Hamstrings'), 'Hamstrings', NULL, 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Hands'), 'Palmar Fascia', 'Hands', 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Lats'), 'Latissimus Dorsi', 'Lats', 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Lower Back'), 'Lower Back', NULL, 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Shoulders'), 'Shoulders', NULL, 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Quadriceps'), 'Quadriceps', 'Quads', 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Upper Back'), 'Rhomboids', NULL, 'https://picsum.photos/400/500'),
    ((SELECT id FROM muscle_groups WHERE name = 'Upper Back'), 'Trapezius', 'Traps', 'https://picsum.photos/400/500');

INSERT INTO muscles (group_id, parent_id, name, simple_name) VALUES
    ((SELECT id FROM muscle_groups WHERE name = 'Arms'), (SELECT id FROM muscles WHERE name = 'Biceps Brachii'), 'Biceps Brachii, Long Head', 'Biceps (Inner Head)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Arms'), (SELECT id FROM muscles WHERE name = 'Biceps Brachii'), 'Biceps Brachii, Short Head', 'Biceps (Outer Head)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Arms'), (SELECT id FROM muscles WHERE name = 'Triceps Brachii'), 'Triceps Brachii, Long Head', 'Triceps (Inner Head)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Arms'), (SELECT id FROM muscles WHERE name = 'Triceps Brachii'), 'Triceps Brachii, Lateral Head', 'Triceps (Outer Head)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Arms'), (SELECT id FROM muscles WHERE name = 'Triceps Brachii'), 'Triceps Brachii, Medial Head', 'Triceps (Middle Head)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Calves'), (SELECT id FROM muscles WHERE name = 'Calves'), 'Tibialis', NULL),
    ((SELECT id FROM muscle_groups WHERE name = 'Calves'), (SELECT id FROM muscles WHERE name = 'Calves'), 'Soleus', NULL),
    ((SELECT id FROM muscle_groups WHERE name = 'Calves'), (SELECT id FROM muscles WHERE name = 'Calves'), 'Gastrocnemius', NULL),
    ((SELECT id FROM muscle_groups WHERE name = 'Core'), (SELECT id FROM muscles WHERE name = 'Abdominals'), 'Rectus Abdominis', 'Upper Abs'),
    ((SELECT id FROM muscle_groups WHERE name = 'Core'), (SELECT id FROM muscles WHERE name = 'Abdominals'), 'Transversus Abdominis', 'Lower Abs'),
    ((SELECT id FROM muscle_groups WHERE name = 'Chest'), (SELECT id FROM muscles WHERE name = 'Chest'), 'Pectoralis Major, Sternocostal Head', 'Chest (Upper)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Chest'), (SELECT id FROM muscles WHERE name = 'Chest'), 'Pectoralis Major, Clavicular Head', 'Chest (Mid & Lower)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Chest'), (SELECT id FROM muscles WHERE name = 'Chest'), 'Serratus Anterior', E'Boxer\'s Muscle'),
    ((SELECT id FROM muscle_groups WHERE name = 'Forearms'), (SELECT id FROM muscles WHERE name = 'Forearms'), 'Wrist Extensors', 'Forearm (Outer)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Forearms'), (SELECT id FROM muscles WHERE name = 'Forearms'), 'Wrist Flexors', 'Forearm (Inner)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Glutes'), (SELECT id FROM muscles WHERE name = 'Glutes'), 'Gluteus Maximus', NULL),
    ((SELECT id FROM muscle_groups WHERE name = 'Glutes'), (SELECT id FROM muscles WHERE name = 'Glutes'), 'Gluteus Medius', NULL),
    ((SELECT id FROM muscle_groups WHERE name = 'Hamstrings'), (SELECT id FROM muscles WHERE name = 'Hamstrings'), 'Medial Hamstrings', 'Hamstrings (Inner)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Hamstrings'), (SELECT id FROM muscles WHERE name = 'Hamstrings'), 'Lateral Hamstrings', 'Hamstrings (Outer)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Quadriceps'), (SELECT id FROM muscles WHERE name = 'Quadriceps'), 'Vastus Intermedius', 'Quadriceps (Deep)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Quadriceps'), (SELECT id FROM muscles WHERE name = 'Quadriceps'), 'Vastus Lateralis', 'Quadriceps (Outer)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Quadriceps'), (SELECT id FROM muscles WHERE name = 'Quadriceps'), 'Vastus Medialis', 'Quadriceps (Inner)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Quadriceps'), (SELECT id FROM muscles WHERE name = 'Quadriceps'), 'Rectus Femoris', 'Quadriceps (Front)'),
    ((SELECT id FROM muscle_groups WHERE name = 'Shoulders'), (SELECT id FROM muscles WHERE name = 'Shoulders'), 'Lateral Deltoid', 'Side Delts'),
    ((SELECT id FROM muscle_groups WHERE name = 'Shoulders'), (SELECT id FROM muscles WHERE name = 'Shoulders'), 'Anterior Deltoid', 'Front Delts'),
    ((SELECT id FROM muscle_groups WHERE name = 'Shoulders'), (SELECT id FROM muscles WHERE name = 'Shoulders'), 'Posterior Deltoid', 'Rear Delts'),
    ((SELECT id FROM muscle_groups WHERE name = 'Upper Back'), (SELECT id FROM muscles WHERE name = 'Trapezius'), 'Trapezius, Superior', 'Upper Traps'),
    ((SELECT id FROM muscle_groups WHERE name = 'Upper Back'), (SELECT id FROM muscles WHERE name = 'Trapezius'), 'Trapezius, Inferior', 'Lower Traps');

INSERT INTO muscles (group_id, parent_id, name, simple_name, image_source) VALUES
    ((SELECT id FROM muscle_groups WHERE name = 'Core'), (SELECT id FROM muscles WHERE name = 'Abdominals'), 'Obliques', NULL, 'https://cdn.muscleandstrength.com/sites/default/files/taxonomy/image/videos/obliques.jpg');

-- INSERT INTO muscles (group_id, name, simple_name) VALUES
--     ('back', 'Erector Spinae', 'Lower Back'),
--     ('back', 'Rhomboids', NULL),
--     ('back', 'Middle Trapezius', 'Traps'),
--     ('core', 'Transverse Abdominus', 'Transverse Abs'),
--     ('shoulders', 'Rotator Cuff', NULL);
--
-- INSERT INTO muscles (parent_id, muscle_group, name, simple_name) VALUES
--     ((SELECT id FROM muscles WHERE name = 'Rotator Cuff'), 'shoulders', 'Infraspinatus', NULL),
--     ((SELECT id FROM muscles WHERE name = 'Rotator Cuff'), 'shoulders', 'Subscapularis', NULL),
--     ((SELECT id FROM muscles WHERE name = 'Rotator Cuff'), 'shoulders', 'Supraspinatus', NULL),
--     ((SELECT id FROM muscles WHERE name = 'Rotator Cuff'), 'shoulders', 'Teres Minor', NULL);
