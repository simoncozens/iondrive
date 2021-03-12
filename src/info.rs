use crate::MyToPyObject;
use crate::ToWrappedPyObject;
use norad::fontinfo::StyleMapStyle;
use norad::IntegerOrFloat;
use norad::NonNegativeIntegerOrFloat;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

impl MyToPyObject for StyleMapStyle {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            StyleMapStyle::Regular => "regular".to_string(),
            StyleMapStyle::Italic => "italic".to_string(),
            StyleMapStyle::Bold => "bold".to_string(),
            StyleMapStyle::BoldItalic => "bold italic".to_string(),
        }
        .to_object(py)
    }
}

impl MyToPyObject for IntegerOrFloat {
    fn to_object(&self, py: Python) -> PyObject {
        if self.is_integer() {
            (self.get() as i64).to_object(py)
        } else {
            self.get().to_object(py)
        }
    }
}

impl MyToPyObject for NonNegativeIntegerOrFloat {
    fn to_object(&self, py: Python) -> PyObject {
        if self.is_integer() {
            (self.get() as i64).to_object(py)
        } else {
            self.get().to_object(py)
        }
    }
}

impl ToWrappedPyObject for norad::FontInfo {
    fn to_wrapped_object(&self, loader: &PyModule, py: Python) -> PyObject {
        let cls = loader.get("Info").unwrap();
        let kwargs = [
            ("ascender", self.ascender.to_object(py)),
            ("capHeight", self.cap_height.to_object(py)),
            ("copyright", self.copyright.to_object(py)),
            ("descender", self.descender.to_object(py)),
            ("familyName", self.family_name.to_object(py)),
            // ("guidelines", self.guidelines.to_object(py)),
            ("italicAngle", self.italic_angle.to_object(py)),
            (
                "macintoshFONDFamilyID",
                self.macintosh_fond_family_id.to_object(py),
            ),
            ("macintoshFONDName", self.macintosh_fond_name.to_object(py)),
            ("note", self.note.to_object(py)),
            // ("openTypeGaspRangeRecords", self.open_type_gasp_range_records.to_object(py)),
            (
                "openTypeHeadCreated",
                self.open_type_head_created.to_object(py),
            ),
            ("openTypeHeadFlags", self.open_type_head_flags.to_object(py)),
            (
                "openTypeHeadLowestRecPPEM",
                self.open_type_head_lowest_rec_ppem.to_object(py),
            ),
            (
                "openTypeHheaAscender",
                self.open_type_hhea_ascender.to_object(py),
            ),
            (
                "openTypeHheaCaretOffset",
                self.open_type_hhea_caret_offset.to_object(py),
            ),
            (
                "openTypeHheaCaretSlopeRise",
                self.open_type_hhea_caret_slope_rise.to_object(py),
            ),
            (
                "openTypeHheaCaretSlopeRun",
                self.open_type_hhea_caret_slope_run.to_object(py),
            ),
            (
                "openTypeHheaDescender",
                self.open_type_hhea_descender.to_object(py),
            ),
            (
                "openTypeHheaLineGap",
                self.open_type_hhea_line_gap.to_object(py),
            ),
            (
                "openTypeNameCompatibleFullName",
                self.open_type_name_compatible_full_name
                    .as_ref()
                    .to_object(py),
            ),
            (
                "openTypeNameDescription",
                self.open_type_name_description.to_object(py),
            ),
            (
                "openTypeNameDesignerURL",
                self.open_type_name_designer_url.to_object(py),
            ),
            (
                "openTypeNameDesigner",
                self.open_type_name_designer.to_object(py),
            ),
            (
                "openTypeNameLicense",
                self.open_type_name_license.to_object(py),
            ),
            (
                "openTypeNameLicenseURL",
                self.open_type_name_license_url.to_object(py),
            ),
            (
                "openTypeNameManufacturer",
                self.open_type_name_manufacturer.to_object(py),
            ),
            (
                "openTypeNameManufacturerURL",
                self.open_type_name_manufacturer_url.to_object(py),
            ),
            (
                "openTypeNamePreferredFamilyName",
                self.open_type_name_preferred_family_name.to_object(py),
            ),
            (
                "openTypeNamePreferredSubfamilyName",
                self.open_type_name_preferred_subfamily_name.to_object(py),
            ),
            (
                "openTypeNameSampleText",
                self.open_type_name_sample_text.to_object(py),
            ),
            (
                "openTypeNameUniqueID",
                self.open_type_name_unique_id.to_object(py),
            ),
            (
                "openTypeNameVersion",
                self.open_type_name_version.to_object(py),
            ),
            (
                "openTypeNameWWSFamilyName",
                self.open_type_name_wws_family_name.to_object(py),
            ),
            (
                "openTypeNameWWSSubfamilyName",
                self.open_type_name_wws_subfamily_name.to_object(py),
            ),
            (
                "openTypeOS2CodePageRanges",
                self.open_type_os2_code_page_ranges.to_object(py),
            ),
            (
                "openTypeOS2Selection",
                self.open_type_os2_selection.to_object(py),
            ),
            (
                "openTypeOS2StrikeoutPosition",
                self.open_type_os2_strikeout_position.to_object(py),
            ),
            (
                "openTypeOS2StrikeoutSize",
                self.open_type_os2_strikeout_size.to_object(py),
            ),
            (
                "openTypeOS2SubscriptXOffset",
                self.open_type_os2_subscript_x_offset.to_object(py),
            ),
            (
                "openTypeOS2SubscriptXSize",
                self.open_type_os2_subscript_x_size.to_object(py),
            ),
            (
                "openTypeOS2SubscriptYOffset",
                self.open_type_os2_subscript_y_offset.to_object(py),
            ),
            (
                "openTypeOS2SubscriptYSize",
                self.open_type_os2_subscript_y_size.to_object(py),
            ),
            (
                "openTypeOS2SuperscriptXOffset",
                self.open_type_os2_superscript_x_offset.to_object(py),
            ),
            (
                "openTypeOS2SuperscriptXSize",
                self.open_type_os2_superscript_x_size.to_object(py),
            ),
            (
                "openTypeOS2SuperscriptYOffset",
                self.open_type_os2_superscript_y_offset.to_object(py),
            ),
            (
                "openTypeOS2SuperscriptYSize",
                self.open_type_os2_superscript_y_size.to_object(py),
            ),
            ("openTypeOS2Type", self.open_type_os2_type.to_object(py)),
            (
                "openTypeOS2TypoAscender",
                self.open_type_os2_typo_ascender.to_object(py),
            ),
            (
                "openTypeOS2TypoDescender",
                self.open_type_os2_typo_descender.to_object(py),
            ),
            (
                "openTypeOS2TypoLineGap",
                self.open_type_os2_typo_line_gap.to_object(py),
            ),
            (
                "openTypeOS2UnicodeRanges",
                self.open_type_os2_unicode_ranges.to_object(py),
            ),
            (
                "openTypeOS2VendorID",
                self.open_type_os2_vendor_id.to_object(py),
            ),
            (
                "openTypeOS2WeightClass",
                self.open_type_os2_weight_class.to_object(py),
            ),
            (
                "openTypeOS2WinAscent",
                self.open_type_os2_win_ascent.to_object(py),
            ),
            (
                "openTypeOS2WinDescent",
                self.open_type_os2_win_descent.to_object(py),
            ),
            (
                "openTypeVheaCaretOffset",
                self.open_type_vhea_caret_offset.to_object(py),
            ),
            (
                "openTypeVheaCaretSlopeRise",
                self.open_type_vhea_caret_slope_rise.to_object(py),
            ),
            (
                "openTypeVheaCaretSlopeRun",
                self.open_type_vhea_caret_slope_run.to_object(py),
            ),
            (
                "openTypeVheaVertTypoAscender",
                self.open_type_vhea_vert_typo_ascender.to_object(py),
            ),
            (
                "openTypeVheaVertTypoDescender",
                self.open_type_vhea_vert_typo_descender.to_object(py),
            ),
            (
                "openTypeVheaVertTypoLineGap",
                self.open_type_vhea_vert_typo_line_gap.to_object(py),
            ),
            (
                "postscriptBlueFuzz",
                self.postscript_blue_fuzz.to_object(py),
            ),
            (
                "postscriptBlueScale",
                self.postscript_blue_scale.to_object(py),
            ),
            (
                "postscriptBlueShift",
                self.postscript_blue_shift.to_object(py),
            ),
            (
                "postscriptBlueValues",
                self.postscript_blue_values.to_object(py),
            ),
            (
                "postscriptDefaultCharacter",
                self.postscript_default_character.to_object(py),
            ),
            (
                "postscriptDefaultWidthX",
                self.postscript_default_width_x.to_object(py),
            ),
            (
                "postscriptFamilyBlues",
                self.postscript_family_blues.to_object(py),
            ),
            (
                "postscriptFamilyOtherBlues",
                self.postscript_family_other_blues.to_object(py),
            ),
            (
                "postscriptFontName",
                self.postscript_font_name.to_object(py),
            ),
            (
                "postscriptForceBold",
                self.postscript_force_bold.to_object(py),
            ),
            (
                "postscriptFullName",
                self.postscript_full_name.to_object(py),
            ),
            (
                "postscriptIsFixedPitch",
                self.postscript_is_fixed_pitch.to_object(py),
            ),
            (
                "postscriptNominalWidthX",
                self.postscript_nominal_width_x.to_object(py),
            ),
            (
                "postscriptOtherBlues",
                self.postscript_other_blues.to_object(py),
            ),
            (
                "postscriptSlantAngle",
                self.postscript_slant_angle.to_object(py),
            ),
            (
                "postscriptStemSnapH",
                self.postscript_stem_snap_h.to_object(py),
            ),
            (
                "postscriptStemSnapV",
                self.postscript_stem_snap_v.to_object(py),
            ),
            (
                "postscriptUnderlinePosition",
                self.postscript_underline_position.to_object(py),
            ),
            (
                "postscriptUnderlineThickness",
                self.postscript_underline_thickness.to_object(py),
            ),
            (
                "postscriptUniqueID",
                self.postscript_unique_id.to_object(py),
            ),
            (
                "postscriptWeightName",
                self.postscript_weight_name.to_object(py),
            ),
            (
                "postscriptWindowsCharacterSet",
                self.postscript_windows_character_set
                    .map(|x| x as u8)
                    .to_object(py),
            ),
            (
                "styleMapFamilyName",
                self.style_map_family_name.to_object(py),
            ),
            ("styleMapStyleName", self.style_map_style_name.to_object(py)),
            ("styleName", self.style_name.to_object(py)),
            ("trademark", self.trademark.to_object(py)),
            ("unitsPerEm", self.units_per_em.to_object(py)),
            ("versionMajor", self.version_major.to_object(py)),
            ("versionMinor", self.version_minor.to_object(py)),
            // ("woffMajorVersion", self.woff_major_version.to_object(py)),
            // ("woffMinorVersion", self.woff_minor_version.to_object(py)),
            ("xHeight", self.x_height.to_object(py)),
            ("year", self.year.to_object(py)),
        ]
        .into_py_dict(py);
        cls.call((), Some(kwargs)).unwrap().into()
    }
}
//     // #[getter]
//     // fn openTypeOS2FamilyClass(&self) -> Option<OS2FamilyClass> {
//     //     self.fontinfo.open_type_os2_family_class
//     // }
//     // #[getter]
//     // fn openTypeOS2Panose(&self) -> Option<OS2Panose> {
//     //     self.fontinfo.open_type_os2_panose
//     // }
//     // #[getter]
//     // fn openTypeOS2WidthClass(&self) -> Option<OS2WidthClass> {
//     //     self.fontinfo.open_type_os2_width_class
//     // }
//     // #[getter]
//     // fn woffMetadataCopyright(&self) -> Option<WoffMetadataCopyright> {
//     //     self.fontinfo.woff_metadata_copyright
//     // }
//     // #[getter]
//     // fn woffMetadataCredits(&self) -> Option<WoffMetadataCredits> {
//     //     self.fontinfo.woff_metadata_credits
//     // }
//     // #[getter]
//     // fn woffMetadataDescription(&self) -> Option<WoffMetadataDescription> {
//     //     self.fontinfo.woff_metadata_description
//     // }
//     // #[getter]
//     // fn woffMetadataExtensions(&self) -> Option<Vec<WoffMetadataExtensionRecord>> {
//     //     self.fontinfo.woff_metadata_extensions
//     // }
//     // #[getter]
//     // fn woffMetadataLicense(&self) -> Option<WoffMetadataLicense> {
//     //     self.fontinfo.woff_metadata_license
//     // }
//     // #[getter]
//     // fn woffMetadataLicensee(&self) -> Option<WoffMetadataLicensee> {
//     //     self.fontinfo.woff_metadata_licensee
//     // }
//     // #[getter]
//     // fn woffMetadataTrademark(&self) -> Option<WoffMetadataTrademark> {
//     //     self.fontinfo.woff_metadata_trademark
//     // }
//     // #[getter]
//     // fn woffMetadataUniqueID(&self) -> Option<WoffMetadataUniqueID> {
//     //     self.fontinfo.woff_metadata_unique_id
//     // }
//     // #[getter]
//     // fn woffMetadataVendor(&self) -> Option<WoffMetadataVendor> {
//     //     self.fontinfo.woff_metadata_vendor
//     // }
// }
