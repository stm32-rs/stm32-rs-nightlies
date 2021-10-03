#[doc = "Register `D3PMR2` reader"]
pub struct R(crate::R<D3PMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D3PMR2` writer"]
pub struct W(crate::W<D3PMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PMR2_SPEC>;
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
impl From<crate::W<D3PMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "D3 Pending Mask on Event input x+32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR34_A {
    #[doc = "0: Interrupt request line is masked"]
    MASKED = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    UNMASKED = 1,
}
impl From<MR34_A> for bool {
    #[inline(always)]
    fn from(variant: MR34_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR34` reader - D3 Pending Mask on Event input x+32"]
pub struct MR34_R(crate::FieldReader<bool, MR34_A>);
impl MR34_R {
    pub(crate) fn new(bits: bool) -> Self {
        MR34_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR34_A {
        match self.bits {
            false => MR34_A::MASKED,
            true => MR34_A::UNMASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == MR34_A::MASKED
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == MR34_A::UNMASKED
    }
}
impl core::ops::Deref for MR34_R {
    type Target = crate::FieldReader<bool, MR34_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MR34` writer - D3 Pending Mask on Event input x+32"]
pub struct MR34_W<'a> {
    w: &'a mut W,
}
impl<'a> MR34_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR34_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR34_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR34_A::UNMASKED)
    }
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
#[doc = "D3 Pending Mask on Event input x+32"]
pub type MR35_A = MR34_A;
#[doc = "Field `MR35` reader - D3 Pending Mask on Event input x+32"]
pub type MR35_R = MR34_R;
#[doc = "Field `MR35` writer - D3 Pending Mask on Event input x+32"]
pub struct MR35_W<'a> {
    w: &'a mut W,
}
impl<'a> MR35_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR35_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR35_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR35_A::UNMASKED)
    }
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
#[doc = "D3 Pending Mask on Event input x+32"]
pub type MR41_A = MR34_A;
#[doc = "Field `MR41` reader - D3 Pending Mask on Event input x+32"]
pub type MR41_R = MR34_R;
#[doc = "Field `MR41` writer - D3 Pending Mask on Event input x+32"]
pub struct MR41_W<'a> {
    w: &'a mut W,
}
impl<'a> MR41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR41_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR41_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR41_A::UNMASKED)
    }
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
#[doc = "D3 Pending Mask on Event input x+32"]
pub type MR48_A = MR34_A;
#[doc = "Field `MR48` reader - D3 Pending Mask on Event input x+32"]
pub type MR48_R = MR34_R;
#[doc = "Field `MR48` writer - D3 Pending Mask on Event input x+32"]
pub struct MR48_W<'a> {
    w: &'a mut W,
}
impl<'a> MR48_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR48_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR48_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR48_A::UNMASKED)
    }
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
#[doc = "D3 Pending Mask on Event input x+32"]
pub type MR49_A = MR34_A;
#[doc = "Field `MR49` reader - D3 Pending Mask on Event input x+32"]
pub type MR49_R = MR34_R;
#[doc = "Field `MR49` writer - D3 Pending Mask on Event input x+32"]
pub struct MR49_W<'a> {
    w: &'a mut W,
}
impl<'a> MR49_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR49_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR49_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR49_A::UNMASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "D3 Pending Mask on Event input x+32"]
pub type MR50_A = MR34_A;
#[doc = "Field `MR50` reader - D3 Pending Mask on Event input x+32"]
pub type MR50_R = MR34_R;
#[doc = "Field `MR50` writer - D3 Pending Mask on Event input x+32"]
pub struct MR50_W<'a> {
    w: &'a mut W,
}
impl<'a> MR50_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR50_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR50_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR50_A::UNMASKED)
    }
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
#[doc = "D3 Pending Mask on Event input x+32"]
pub type MR51_A = MR34_A;
#[doc = "Field `MR51` reader - D3 Pending Mask on Event input x+32"]
pub type MR51_R = MR34_R;
#[doc = "Field `MR51` writer - D3 Pending Mask on Event input x+32"]
pub struct MR51_W<'a> {
    w: &'a mut W,
}
impl<'a> MR51_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR51_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR51_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR51_A::UNMASKED)
    }
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
#[doc = "D3 Pending Mask on Event input x+32"]
pub type MR52_A = MR34_A;
#[doc = "Field `MR52` reader - D3 Pending Mask on Event input x+32"]
pub type MR52_R = MR34_R;
#[doc = "Field `MR52` writer - D3 Pending Mask on Event input x+32"]
pub struct MR52_W<'a> {
    w: &'a mut W,
}
impl<'a> MR52_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR52_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR52_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR52_A::UNMASKED)
    }
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
#[doc = "D3 Pending Mask on Event input x+32"]
pub type MR53_A = MR34_A;
#[doc = "Field `MR53` reader - D3 Pending Mask on Event input x+32"]
pub type MR53_R = MR34_R;
#[doc = "Field `MR53` writer - D3 Pending Mask on Event input x+32"]
pub struct MR53_W<'a> {
    w: &'a mut W,
}
impl<'a> MR53_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR53_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR53_A::MASKED)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR53_A::UNMASKED)
    }
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
impl R {
    #[doc = "Bit 2 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr34(&mut self) -> MR34_W {
        MR34_W { w: self }
    }
    #[doc = "Bit 3 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr35(&mut self) -> MR35_W {
        MR35_W { w: self }
    }
    #[doc = "Bit 9 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr41(&mut self) -> MR41_W {
        MR41_W { w: self }
    }
    #[doc = "Bit 16 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr48(&mut self) -> MR48_W {
        MR48_W { w: self }
    }
    #[doc = "Bit 17 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr49(&mut self) -> MR49_W {
        MR49_W { w: self }
    }
    #[doc = "Bit 18 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr50(&mut self) -> MR50_W {
        MR50_W { w: self }
    }
    #[doc = "Bit 19 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr51(&mut self) -> MR51_W {
        MR51_W { w: self }
    }
    #[doc = "Bit 20 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr52(&mut self) -> MR52_W {
        MR52_W { w: self }
    }
    #[doc = "Bit 21 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr53(&mut self) -> MR53_W {
        MR53_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI D3 pending mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d3pmr2](index.html) module"]
pub struct D3PMR2_SPEC;
impl crate::RegisterSpec for D3PMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d3pmr2::R](R) reader structure"]
impl crate::Readable for D3PMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d3pmr2::W](W) writer structure"]
impl crate::Writable for D3PMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D3PMR2 to value 0"]
impl crate::Resettable for D3PMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
