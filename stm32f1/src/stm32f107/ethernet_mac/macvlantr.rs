#[doc = "Register `MACVLANTR` reader"]
pub type R = crate::R<MACVLANTRrs>;
#[doc = "Register `MACVLANTR` writer"]
pub type W = crate::W<MACVLANTRrs>;
#[doc = "Field `VLANTI` reader - VLAN tag identifier (for receive frames)"]
pub type VLANTI_R = crate::FieldReader<u16>;
#[doc = "Field `VLANTI` writer - VLAN tag identifier (for receive frames)"]
pub type VLANTI_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
#[doc = "12-bit VLAN tag comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLANTC {
    #[doc = "0: Full 16 bit VLAN identifiers are used for comparison and filtering"]
    Vlantc16 = 0,
    #[doc = "1: 12 bit VLAN identifies are used for comparison and filtering"]
    Vlantc12 = 1,
}
impl From<VLANTC> for bool {
    #[inline(always)]
    fn from(variant: VLANTC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLANTC` reader - 12-bit VLAN tag comparison"]
pub type VLANTC_R = crate::BitReader<VLANTC>;
impl VLANTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VLANTC {
        match self.bits {
            false => VLANTC::Vlantc16,
            true => VLANTC::Vlantc12,
        }
    }
    #[doc = "Full 16 bit VLAN identifiers are used for comparison and filtering"]
    #[inline(always)]
    pub fn is_vlantc16(&self) -> bool {
        *self == VLANTC::Vlantc16
    }
    #[doc = "12 bit VLAN identifies are used for comparison and filtering"]
    #[inline(always)]
    pub fn is_vlantc12(&self) -> bool {
        *self == VLANTC::Vlantc12
    }
}
#[doc = "Field `VLANTC` writer - 12-bit VLAN tag comparison"]
pub type VLANTC_W<'a, REG> = crate::BitWriter<'a, REG, VLANTC>;
impl<'a, REG> VLANTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full 16 bit VLAN identifiers are used for comparison and filtering"]
    #[inline(always)]
    pub fn vlantc16(self) -> &'a mut crate::W<REG> {
        self.variant(VLANTC::Vlantc16)
    }
    #[doc = "12 bit VLAN identifies are used for comparison and filtering"]
    #[inline(always)]
    pub fn vlantc12(self) -> &'a mut crate::W<REG> {
        self.variant(VLANTC::Vlantc12)
    }
}
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    #[must_use]
    pub fn vlanti(&mut self) -> VLANTI_W<MACVLANTRrs> {
        VLANTI_W::new(self, 0)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    #[must_use]
    pub fn vlantc(&mut self) -> VLANTC_W<MACVLANTRrs> {
        VLANTC_W::new(self, 16)
    }
}
#[doc = "Ethernet MAC VLAN tag register (ETH_MACVLANTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvlantr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvlantr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVLANTRrs;
impl crate::RegisterSpec for MACVLANTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvlantr::R`](R) reader structure"]
impl crate::Readable for MACVLANTRrs {}
#[doc = "`write(|w| ..)` method takes [`macvlantr::W`](W) writer structure"]
impl crate::Writable for MACVLANTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACVLANTR to value 0"]
impl crate::Resettable for MACVLANTRrs {
    const RESET_VALUE: u32 = 0;
}
