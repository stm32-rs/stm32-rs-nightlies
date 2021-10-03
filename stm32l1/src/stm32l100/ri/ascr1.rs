#[doc = "Register `ASCR1` reader"]
pub struct R(crate::R<ASCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASCR1` writer"]
pub struct W(crate::W<ASCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ASCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCM` reader - Switch control mode"]
pub struct SCM_R(crate::FieldReader<bool, bool>);
impl SCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCM` writer - Switch control mode"]
pub struct SCM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `CH30GR11_4` reader - Analog switch control"]
pub struct CH30GR11_4_R(crate::FieldReader<bool, bool>);
impl CH30GR11_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH30GR11_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH30GR11_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH30GR11_4` writer - Analog switch control"]
pub struct CH30GR11_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH30GR11_4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CH29GR11_3` reader - Analog switch control"]
pub struct CH29GR11_3_R(crate::FieldReader<bool, bool>);
impl CH29GR11_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH29GR11_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH29GR11_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH29GR11_3` writer - Analog switch control"]
pub struct CH29GR11_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH29GR11_3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `CH28GR11_2` reader - Analog switch control"]
pub struct CH28GR11_2_R(crate::FieldReader<bool, bool>);
impl CH28GR11_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH28GR11_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH28GR11_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH28GR11_2` writer - Analog switch control"]
pub struct CH28GR11_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH28GR11_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CH27GR11_1` reader - Analog switch control"]
pub struct CH27GR11_1_R(crate::FieldReader<bool, bool>);
impl CH27GR11_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH27GR11_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH27GR11_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH27GR11_1` writer - Analog switch control"]
pub struct CH27GR11_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH27GR11_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `VCOMP` reader - ADC analog switch selection for internal node to comparator 1"]
pub struct VCOMP_R(crate::FieldReader<bool, bool>);
impl VCOMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOMP` writer - ADC analog switch selection for internal node to comparator 1"]
pub struct VCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CH25` reader - Analog I/O switch control of channel CH25"]
pub struct CH25_R(crate::FieldReader<bool, bool>);
impl CH25_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH25` writer - Analog I/O switch control of channel CH25"]
pub struct CH25_W<'a> {
    w: &'a mut W,
}
impl<'a> CH25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CH24` reader - Analog I/O switch control of channel CH24"]
pub struct CH24_R(crate::FieldReader<bool, bool>);
impl CH24_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH24` writer - Analog I/O switch control of channel CH24"]
pub struct CH24_W<'a> {
    w: &'a mut W,
}
impl<'a> CH24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CH23` reader - Analog I/O switch control of channel CH23"]
pub struct CH23_R(crate::FieldReader<bool, bool>);
impl CH23_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH23` writer - Analog I/O switch control of channel CH23"]
pub struct CH23_W<'a> {
    w: &'a mut W,
}
impl<'a> CH23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CH22` reader - Analog I/O switch control of channel CH22"]
pub struct CH22_R(crate::FieldReader<bool, bool>);
impl CH22_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH22` writer - Analog I/O switch control of channel CH22"]
pub struct CH22_W<'a> {
    w: &'a mut W,
}
impl<'a> CH22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CH21GR7_4` reader - Analog switch control"]
pub struct CH21GR7_4_R(crate::FieldReader<bool, bool>);
impl CH21GR7_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH21GR7_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH21GR7_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH21GR7_4` writer - Analog switch control"]
pub struct CH21GR7_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH21GR7_4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CH20GR7_3` reader - Analog switch control"]
pub struct CH20GR7_3_R(crate::FieldReader<bool, bool>);
impl CH20GR7_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH20GR7_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH20GR7_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH20GR7_3` writer - Analog switch control"]
pub struct CH20GR7_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH20GR7_3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CH19GR7_2` reader - Analog switch control"]
pub struct CH19GR7_2_R(crate::FieldReader<bool, bool>);
impl CH19GR7_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH19GR7_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH19GR7_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH19GR7_2` writer - Analog switch control"]
pub struct CH19GR7_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH19GR7_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CH18GR7_1` reader - Analog switch control"]
pub struct CH18GR7_1_R(crate::FieldReader<bool, bool>);
impl CH18GR7_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH18GR7_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH18GR7_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH18GR7_1` writer - Analog switch control"]
pub struct CH18GR7_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH18GR7_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CH31GR7_1` reader - Analog switch control"]
pub struct CH31GR7_1_R(crate::FieldReader<bool, bool>);
impl CH31GR7_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH31GR7_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH31GR7_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH31GR7_1` writer - Analog switch control"]
pub struct CH31GR7_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH31GR7_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CH15GR9_2` reader - Analog switch control"]
pub struct CH15GR9_2_R(crate::FieldReader<bool, bool>);
impl CH15GR9_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH15GR9_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH15GR9_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH15GR9_2` writer - Analog switch control"]
pub struct CH15GR9_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15GR9_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CH14GR9_1` reader - Analog switch control"]
pub struct CH14GR9_1_R(crate::FieldReader<bool, bool>);
impl CH14GR9_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH14GR9_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH14GR9_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH14GR9_1` writer - Analog switch control"]
pub struct CH14GR9_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14GR9_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CH13GR8_4` reader - Analog switch control"]
pub struct CH13GR8_4_R(crate::FieldReader<bool, bool>);
impl CH13GR8_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH13GR8_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH13GR8_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH13GR8_4` writer - Analog switch control"]
pub struct CH13GR8_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13GR8_4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CH12GR8_3` reader - Analog switch control"]
pub struct CH12GR8_3_R(crate::FieldReader<bool, bool>);
impl CH12GR8_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH12GR8_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH12GR8_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH12GR8_3` writer - Analog switch control"]
pub struct CH12GR8_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12GR8_3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CH11GR8_2` reader - Analog switch control"]
pub struct CH11GR8_2_R(crate::FieldReader<bool, bool>);
impl CH11GR8_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH11GR8_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH11GR8_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH11GR8_2` writer - Analog switch control"]
pub struct CH11GR8_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11GR8_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CH10GR8_1` reader - Analog switch control"]
pub struct CH10GR8_1_R(crate::FieldReader<bool, bool>);
impl CH10GR8_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH10GR8_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH10GR8_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH10GR8_1` writer - Analog switch control"]
pub struct CH10GR8_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10GR8_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CH9GR3_2` reader - Analog switch control"]
pub struct CH9GR3_2_R(crate::FieldReader<bool, bool>);
impl CH9GR3_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH9GR3_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH9GR3_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH9GR3_2` writer - Analog switch control"]
pub struct CH9GR3_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9GR3_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CH8GR3_1` reader - Analog switch control"]
pub struct CH8GR3_1_R(crate::FieldReader<bool, bool>);
impl CH8GR3_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH8GR3_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH8GR3_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH8GR3_1` writer - Analog switch control"]
pub struct CH8GR3_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8GR3_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CH7GR2_2` reader - Analog switch control"]
pub struct CH7GR2_2_R(crate::FieldReader<bool, bool>);
impl CH7GR2_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH7GR2_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7GR2_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7GR2_2` writer - Analog switch control"]
pub struct CH7GR2_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7GR2_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CH6GR2_1` reader - Analog switch control"]
pub struct CH6GR2_1_R(crate::FieldReader<bool, bool>);
impl CH6GR2_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH6GR2_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6GR2_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6GR2_1` writer - Analog switch control"]
pub struct CH6GR2_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6GR2_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `COMP1_SW1` reader - Comparator 1 analog switch"]
pub struct COMP1_SW1_R(crate::FieldReader<bool, bool>);
impl COMP1_SW1_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_SW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_SW1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1_SW1` writer - Comparator 1 analog switch"]
pub struct COMP1_SW1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_SW1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CH31GR11_5` reader - Analog switch control"]
pub struct CH31GR11_5_R(crate::FieldReader<bool, bool>);
impl CH31GR11_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH31GR11_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH31GR11_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH31GR11_5` writer - Analog switch control"]
pub struct CH31GR11_5_W<'a> {
    w: &'a mut W,
}
impl<'a> CH31GR11_5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CH3GR1_4` reader - Analog switch control"]
pub struct CH3GR1_4_R(crate::FieldReader<bool, bool>);
impl CH3GR1_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3GR1_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3GR1_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3GR1_4` writer - Analog switch control"]
pub struct CH3GR1_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3GR1_4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CH2GR1_3` reader - Analog switch control"]
pub struct CH2GR1_3_R(crate::FieldReader<bool, bool>);
impl CH2GR1_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2GR1_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2GR1_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2GR1_3` writer - Analog switch control"]
pub struct CH2GR1_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2GR1_3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CH1GR1_2` reader - Analog switch control"]
pub struct CH1GR1_2_R(crate::FieldReader<bool, bool>);
impl CH1GR1_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1GR1_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1GR1_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1GR1_2` writer - Analog switch control"]
pub struct CH1GR1_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1GR1_2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CH0GR1_1` reader - Analog switch control"]
pub struct CH0GR1_1_R(crate::FieldReader<bool, bool>);
impl CH0GR1_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0GR1_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0GR1_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0GR1_1` writer - Analog switch control"]
pub struct CH0GR1_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0GR1_1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Switch control mode"]
    #[inline(always)]
    pub fn scm(&self) -> SCM_R {
        SCM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Analog switch control"]
    #[inline(always)]
    pub fn ch30gr11_4(&self) -> CH30GR11_4_R {
        CH30GR11_4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Analog switch control"]
    #[inline(always)]
    pub fn ch29gr11_3(&self) -> CH29GR11_3_R {
        CH29GR11_3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Analog switch control"]
    #[inline(always)]
    pub fn ch28gr11_2(&self) -> CH28GR11_2_R {
        CH28GR11_2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Analog switch control"]
    #[inline(always)]
    pub fn ch27gr11_1(&self) -> CH27GR11_1_R {
        CH27GR11_1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ADC analog switch selection for internal node to comparator 1"]
    #[inline(always)]
    pub fn vcomp(&self) -> VCOMP_R {
        VCOMP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Analog I/O switch control of channel CH25"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Analog I/O switch control of channel CH24"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Analog I/O switch control of channel CH23"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Analog I/O switch control of channel CH22"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Analog switch control"]
    #[inline(always)]
    pub fn ch21gr7_4(&self) -> CH21GR7_4_R {
        CH21GR7_4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Analog switch control"]
    #[inline(always)]
    pub fn ch20gr7_3(&self) -> CH20GR7_3_R {
        CH20GR7_3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Analog switch control"]
    #[inline(always)]
    pub fn ch19gr7_2(&self) -> CH19GR7_2_R {
        CH19GR7_2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Analog switch control"]
    #[inline(always)]
    pub fn ch18gr7_1(&self) -> CH18GR7_1_R {
        CH18GR7_1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr7_1(&self) -> CH31GR7_1_R {
        CH31GR7_1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Analog switch control"]
    #[inline(always)]
    pub fn ch15gr9_2(&self) -> CH15GR9_2_R {
        CH15GR9_2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Analog switch control"]
    #[inline(always)]
    pub fn ch14gr9_1(&self) -> CH14GR9_1_R {
        CH14GR9_1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Analog switch control"]
    #[inline(always)]
    pub fn ch13gr8_4(&self) -> CH13GR8_4_R {
        CH13GR8_4_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Analog switch control"]
    #[inline(always)]
    pub fn ch12gr8_3(&self) -> CH12GR8_3_R {
        CH12GR8_3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Analog switch control"]
    #[inline(always)]
    pub fn ch11gr8_2(&self) -> CH11GR8_2_R {
        CH11GR8_2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Analog switch control"]
    #[inline(always)]
    pub fn ch10gr8_1(&self) -> CH10GR8_1_R {
        CH10GR8_1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog switch control"]
    #[inline(always)]
    pub fn ch9gr3_2(&self) -> CH9GR3_2_R {
        CH9GR3_2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Analog switch control"]
    #[inline(always)]
    pub fn ch8gr3_1(&self) -> CH8GR3_1_R {
        CH8GR3_1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog switch control"]
    #[inline(always)]
    pub fn ch7gr2_2(&self) -> CH7GR2_2_R {
        CH7GR2_2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog switch control"]
    #[inline(always)]
    pub fn ch6gr2_1(&self) -> CH6GR2_1_R {
        CH6GR2_1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Comparator 1 analog switch"]
    #[inline(always)]
    pub fn comp1_sw1(&self) -> COMP1_SW1_R {
        COMP1_SW1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr11_5(&self) -> CH31GR11_5_R {
        CH31GR11_5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Analog switch control"]
    #[inline(always)]
    pub fn ch3gr1_4(&self) -> CH3GR1_4_R {
        CH3GR1_4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Analog switch control"]
    #[inline(always)]
    pub fn ch2gr1_3(&self) -> CH2GR1_3_R {
        CH2GR1_3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog switch control"]
    #[inline(always)]
    pub fn ch1gr1_2(&self) -> CH1GR1_2_R {
        CH1GR1_2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Analog switch control"]
    #[inline(always)]
    pub fn ch0gr1_1(&self) -> CH0GR1_1_R {
        CH0GR1_1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Switch control mode"]
    #[inline(always)]
    pub fn scm(&mut self) -> SCM_W {
        SCM_W { w: self }
    }
    #[doc = "Bit 30 - Analog switch control"]
    #[inline(always)]
    pub fn ch30gr11_4(&mut self) -> CH30GR11_4_W {
        CH30GR11_4_W { w: self }
    }
    #[doc = "Bit 29 - Analog switch control"]
    #[inline(always)]
    pub fn ch29gr11_3(&mut self) -> CH29GR11_3_W {
        CH29GR11_3_W { w: self }
    }
    #[doc = "Bit 28 - Analog switch control"]
    #[inline(always)]
    pub fn ch28gr11_2(&mut self) -> CH28GR11_2_W {
        CH28GR11_2_W { w: self }
    }
    #[doc = "Bit 27 - Analog switch control"]
    #[inline(always)]
    pub fn ch27gr11_1(&mut self) -> CH27GR11_1_W {
        CH27GR11_1_W { w: self }
    }
    #[doc = "Bit 26 - ADC analog switch selection for internal node to comparator 1"]
    #[inline(always)]
    pub fn vcomp(&mut self) -> VCOMP_W {
        VCOMP_W { w: self }
    }
    #[doc = "Bit 25 - Analog I/O switch control of channel CH25"]
    #[inline(always)]
    pub fn ch25(&mut self) -> CH25_W {
        CH25_W { w: self }
    }
    #[doc = "Bit 24 - Analog I/O switch control of channel CH24"]
    #[inline(always)]
    pub fn ch24(&mut self) -> CH24_W {
        CH24_W { w: self }
    }
    #[doc = "Bit 23 - Analog I/O switch control of channel CH23"]
    #[inline(always)]
    pub fn ch23(&mut self) -> CH23_W {
        CH23_W { w: self }
    }
    #[doc = "Bit 22 - Analog I/O switch control of channel CH22"]
    #[inline(always)]
    pub fn ch22(&mut self) -> CH22_W {
        CH22_W { w: self }
    }
    #[doc = "Bit 21 - Analog switch control"]
    #[inline(always)]
    pub fn ch21gr7_4(&mut self) -> CH21GR7_4_W {
        CH21GR7_4_W { w: self }
    }
    #[doc = "Bit 20 - Analog switch control"]
    #[inline(always)]
    pub fn ch20gr7_3(&mut self) -> CH20GR7_3_W {
        CH20GR7_3_W { w: self }
    }
    #[doc = "Bit 19 - Analog switch control"]
    #[inline(always)]
    pub fn ch19gr7_2(&mut self) -> CH19GR7_2_W {
        CH19GR7_2_W { w: self }
    }
    #[doc = "Bit 18 - Analog switch control"]
    #[inline(always)]
    pub fn ch18gr7_1(&mut self) -> CH18GR7_1_W {
        CH18GR7_1_W { w: self }
    }
    #[doc = "Bit 16 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr7_1(&mut self) -> CH31GR7_1_W {
        CH31GR7_1_W { w: self }
    }
    #[doc = "Bit 15 - Analog switch control"]
    #[inline(always)]
    pub fn ch15gr9_2(&mut self) -> CH15GR9_2_W {
        CH15GR9_2_W { w: self }
    }
    #[doc = "Bit 14 - Analog switch control"]
    #[inline(always)]
    pub fn ch14gr9_1(&mut self) -> CH14GR9_1_W {
        CH14GR9_1_W { w: self }
    }
    #[doc = "Bit 13 - Analog switch control"]
    #[inline(always)]
    pub fn ch13gr8_4(&mut self) -> CH13GR8_4_W {
        CH13GR8_4_W { w: self }
    }
    #[doc = "Bit 12 - Analog switch control"]
    #[inline(always)]
    pub fn ch12gr8_3(&mut self) -> CH12GR8_3_W {
        CH12GR8_3_W { w: self }
    }
    #[doc = "Bit 11 - Analog switch control"]
    #[inline(always)]
    pub fn ch11gr8_2(&mut self) -> CH11GR8_2_W {
        CH11GR8_2_W { w: self }
    }
    #[doc = "Bit 10 - Analog switch control"]
    #[inline(always)]
    pub fn ch10gr8_1(&mut self) -> CH10GR8_1_W {
        CH10GR8_1_W { w: self }
    }
    #[doc = "Bit 9 - Analog switch control"]
    #[inline(always)]
    pub fn ch9gr3_2(&mut self) -> CH9GR3_2_W {
        CH9GR3_2_W { w: self }
    }
    #[doc = "Bit 8 - Analog switch control"]
    #[inline(always)]
    pub fn ch8gr3_1(&mut self) -> CH8GR3_1_W {
        CH8GR3_1_W { w: self }
    }
    #[doc = "Bit 7 - Analog switch control"]
    #[inline(always)]
    pub fn ch7gr2_2(&mut self) -> CH7GR2_2_W {
        CH7GR2_2_W { w: self }
    }
    #[doc = "Bit 6 - Analog switch control"]
    #[inline(always)]
    pub fn ch6gr2_1(&mut self) -> CH6GR2_1_W {
        CH6GR2_1_W { w: self }
    }
    #[doc = "Bit 5 - Comparator 1 analog switch"]
    #[inline(always)]
    pub fn comp1_sw1(&mut self) -> COMP1_SW1_W {
        COMP1_SW1_W { w: self }
    }
    #[doc = "Bit 4 - Analog switch control"]
    #[inline(always)]
    pub fn ch31gr11_5(&mut self) -> CH31GR11_5_W {
        CH31GR11_5_W { w: self }
    }
    #[doc = "Bit 3 - Analog switch control"]
    #[inline(always)]
    pub fn ch3gr1_4(&mut self) -> CH3GR1_4_W {
        CH3GR1_4_W { w: self }
    }
    #[doc = "Bit 2 - Analog switch control"]
    #[inline(always)]
    pub fn ch2gr1_3(&mut self) -> CH2GR1_3_W {
        CH2GR1_3_W { w: self }
    }
    #[doc = "Bit 1 - Analog switch control"]
    #[inline(always)]
    pub fn ch1gr1_2(&mut self) -> CH1GR1_2_W {
        CH1GR1_2_W { w: self }
    }
    #[doc = "Bit 0 - Analog switch control"]
    #[inline(always)]
    pub fn ch0gr1_1(&mut self) -> CH0GR1_1_W {
        CH0GR1_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI analog switches control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ascr1](index.html) module"]
pub struct ASCR1_SPEC;
impl crate::RegisterSpec for ASCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ascr1::R](R) reader structure"]
impl crate::Readable for ASCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ascr1::W](W) writer structure"]
impl crate::Writable for ASCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASCR1 to value 0"]
impl crate::Resettable for ASCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
