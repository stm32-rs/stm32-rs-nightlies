#[doc = "Register `OAR2` reader"]
pub type R = crate::R<OAR2rs>;
#[doc = "Register `OAR2` writer"]
pub type W = crate::W<OAR2rs>;
#[doc = "Field `OA2` reader - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
pub type OA2_R = crate::FieldReader;
#[doc = "Field `OA2` writer - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
pub type OA2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OA2MSK {
    #[doc = "0: No mask"]
    NoMask = 0,
    #[doc = "1: OA2\\[1\\]
is masked and don’t care. Only OA2\\[7:2\\]
are compared"]
    Mask1 = 1,
    #[doc = "2: OA2\\[2:1\\]
are masked and don’t care. Only OA2\\[7:3\\]
are compared"]
    Mask2 = 2,
    #[doc = "3: OA2\\[3:1\\]
are masked and don’t care. Only OA2\\[7:4\\]
are compared"]
    Mask3 = 3,
    #[doc = "4: OA2\\[4:1\\]
are masked and don’t care. Only OA2\\[7:5\\]
are compared"]
    Mask4 = 4,
    #[doc = "5: OA2\\[5:1\\]
are masked and don’t care. Only OA2\\[7:6\\]
are compared"]
    Mask5 = 5,
    #[doc = "6: OA2\\[6:1\\]
are masked and don’t care. Only OA2\\[7\\]
is compared."]
    Mask6 = 6,
    #[doc = "7: OA2\\[7:1\\]
are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    Mask7 = 7,
}
impl From<OA2MSK> for u8 {
    #[inline(always)]
    fn from(variant: OA2MSK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OA2MSK {
    type Ux = u8;
}
#[doc = "Field `OA2MSK` reader - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
pub type OA2MSK_R = crate::FieldReader<OA2MSK>;
impl OA2MSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OA2MSK {
        match self.bits {
            0 => OA2MSK::NoMask,
            1 => OA2MSK::Mask1,
            2 => OA2MSK::Mask2,
            3 => OA2MSK::Mask3,
            4 => OA2MSK::Mask4,
            5 => OA2MSK::Mask5,
            6 => OA2MSK::Mask6,
            7 => OA2MSK::Mask7,
            _ => unreachable!(),
        }
    }
    #[doc = "No mask"]
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        *self == OA2MSK::NoMask
    }
    #[doc = "OA2\\[1\\]
is masked and don’t care. Only OA2\\[7:2\\]
are compared"]
    #[inline(always)]
    pub fn is_mask1(&self) -> bool {
        *self == OA2MSK::Mask1
    }
    #[doc = "OA2\\[2:1\\]
are masked and don’t care. Only OA2\\[7:3\\]
are compared"]
    #[inline(always)]
    pub fn is_mask2(&self) -> bool {
        *self == OA2MSK::Mask2
    }
    #[doc = "OA2\\[3:1\\]
are masked and don’t care. Only OA2\\[7:4\\]
are compared"]
    #[inline(always)]
    pub fn is_mask3(&self) -> bool {
        *self == OA2MSK::Mask3
    }
    #[doc = "OA2\\[4:1\\]
are masked and don’t care. Only OA2\\[7:5\\]
are compared"]
    #[inline(always)]
    pub fn is_mask4(&self) -> bool {
        *self == OA2MSK::Mask4
    }
    #[doc = "OA2\\[5:1\\]
are masked and don’t care. Only OA2\\[7:6\\]
are compared"]
    #[inline(always)]
    pub fn is_mask5(&self) -> bool {
        *self == OA2MSK::Mask5
    }
    #[doc = "OA2\\[6:1\\]
are masked and don’t care. Only OA2\\[7\\]
is compared."]
    #[inline(always)]
    pub fn is_mask6(&self) -> bool {
        *self == OA2MSK::Mask6
    }
    #[doc = "OA2\\[7:1\\]
are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    #[inline(always)]
    pub fn is_mask7(&self) -> bool {
        *self == OA2MSK::Mask7
    }
}
#[doc = "Field `OA2MSK` writer - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
pub type OA2MSK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, OA2MSK>;
impl<'a, REG> OA2MSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No mask"]
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::NoMask)
    }
    #[doc = "OA2\\[1\\]
is masked and don’t care. Only OA2\\[7:2\\]
are compared"]
    #[inline(always)]
    pub fn mask1(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask1)
    }
    #[doc = "OA2\\[2:1\\]
are masked and don’t care. Only OA2\\[7:3\\]
are compared"]
    #[inline(always)]
    pub fn mask2(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask2)
    }
    #[doc = "OA2\\[3:1\\]
are masked and don’t care. Only OA2\\[7:4\\]
are compared"]
    #[inline(always)]
    pub fn mask3(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask3)
    }
    #[doc = "OA2\\[4:1\\]
are masked and don’t care. Only OA2\\[7:5\\]
are compared"]
    #[inline(always)]
    pub fn mask4(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask4)
    }
    #[doc = "OA2\\[5:1\\]
are masked and don’t care. Only OA2\\[7:6\\]
are compared"]
    #[inline(always)]
    pub fn mask5(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask5)
    }
    #[doc = "OA2\\[6:1\\]
are masked and don’t care. Only OA2\\[7\\]
is compared."]
    #[inline(always)]
    pub fn mask6(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask6)
    }
    #[doc = "OA2\\[7:1\\]
are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    #[inline(always)]
    pub fn mask7(self) -> &'a mut crate::W<REG> {
        self.variant(OA2MSK::Mask7)
    }
}
#[doc = "Own Address 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OA2EN {
    #[doc = "0: Own address 2 disabled. The received slave address OA2 is NACKed"]
    Disabled = 0,
    #[doc = "1: Own address 2 enabled. The received slave address OA2 is ACKed"]
    Enabled = 1,
}
impl From<OA2EN> for bool {
    #[inline(always)]
    fn from(variant: OA2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA2EN` reader - Own Address 2 enable"]
pub type OA2EN_R = crate::BitReader<OA2EN>;
impl OA2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OA2EN {
        match self.bits {
            false => OA2EN::Disabled,
            true => OA2EN::Enabled,
        }
    }
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OA2EN::Disabled
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OA2EN::Enabled
    }
}
#[doc = "Field `OA2EN` writer - Own Address 2 enable"]
pub type OA2EN_W<'a, REG> = crate::BitWriter<'a, REG, OA2EN>;
impl<'a, REG> OA2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OA2EN::Disabled)
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OA2EN::Enabled)
    }
}
impl R {
    #[doc = "Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    #[must_use]
    pub fn oa2(&mut self) -> OA2_W<OAR2rs> {
        OA2_W::new(self, 1)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
    #[inline(always)]
    #[must_use]
    pub fn oa2msk(&mut self) -> OA2MSK_W<OAR2rs> {
        OA2MSK_W::new(self, 8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn oa2en(&mut self) -> OA2EN_W<OAR2rs> {
        OA2EN_W::new(self, 15)
    }
}
#[doc = "Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OAR2rs;
impl crate::RegisterSpec for OAR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar2::R`](R) reader structure"]
impl crate::Readable for OAR2rs {}
#[doc = "`write(|w| ..)` method takes [`oar2::W`](W) writer structure"]
impl crate::Writable for OAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR2 to value 0"]
impl crate::Resettable for OAR2rs {
    const RESET_VALUE: u32 = 0;
}
