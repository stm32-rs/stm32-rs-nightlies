#[doc = "Register `MACA2HR` reader"]
pub type R = crate::R<MACA2HRrs>;
#[doc = "Register `MACA2HR` writer"]
pub type W = crate::W<MACA2HRrs>;
#[doc = "Field `MACA2H` reader - Ethernet MAC address 2 high register"]
pub type MACA2H_R = crate::FieldReader<u16>;
#[doc = "Field `MACA2H` writer - Ethernet MAC address 2 high register"]
pub type MACA2H_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask byte control"]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask byte control"]
pub type MBC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
#[doc = "Source address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SA {
    #[doc = "0: This address is used for comparison with DA fields of the received frame"]
    Destination = 0,
    #[doc = "1: This address is used for comparison with SA fields of received frames"]
    Source = 1,
}
impl From<SA> for bool {
    #[inline(always)]
    fn from(variant: SA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SA` reader - Source address"]
pub type SA_R = crate::BitReader<SA>;
impl SA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SA {
        match self.bits {
            false => SA::Destination,
            true => SA::Source,
        }
    }
    #[doc = "This address is used for comparison with DA fields of the received frame"]
    #[inline(always)]
    pub fn is_destination(&self) -> bool {
        *self == SA::Destination
    }
    #[doc = "This address is used for comparison with SA fields of received frames"]
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == SA::Source
    }
}
#[doc = "Field `SA` writer - Source address"]
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG, SA>;
impl<'a, REG> SA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This address is used for comparison with DA fields of the received frame"]
    #[inline(always)]
    pub fn destination(self) -> &'a mut crate::W<REG> {
        self.variant(SA::Destination)
    }
    #[doc = "This address is used for comparison with SA fields of received frames"]
    #[inline(always)]
    pub fn source(self) -> &'a mut crate::W<REG> {
        self.variant(SA::Source)
    }
}
#[doc = "Address enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AE {
    #[doc = "0: Address filters ignore this address"]
    Disabled = 0,
    #[doc = "1: Address filters use this address"]
    Enabled = 1,
}
impl From<AE> for bool {
    #[inline(always)]
    fn from(variant: AE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AE` reader - Address enable"]
pub type AE_R = crate::BitReader<AE>;
impl AE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AE {
        match self.bits {
            false => AE::Disabled,
            true => AE::Enabled,
        }
    }
    #[doc = "Address filters ignore this address"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AE::Disabled
    }
    #[doc = "Address filters use this address"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AE::Enabled
    }
}
#[doc = "Field `AE` writer - Address enable"]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG, AE>;
impl<'a, REG> AE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address filters ignore this address"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AE::Disabled)
    }
    #[doc = "Address filters use this address"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AE::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:15 - Ethernet MAC address 2 high register"]
    #[inline(always)]
    pub fn maca2h(&self) -> MACA2H_R {
        MACA2H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ethernet MAC address 2 high register"]
    #[inline(always)]
    #[must_use]
    pub fn maca2h(&mut self) -> MACA2H_W<MACA2HRrs> {
        MACA2H_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<MACA2HRrs> {
        MBC_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<MACA2HRrs> {
        SA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<MACA2HRrs> {
        AE_W::new(self, 31)
    }
}
#[doc = "Ethernet MAC address 2 high register (ETH_MACA2HR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA2HRrs;
impl crate::RegisterSpec for MACA2HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2hr::R`](R) reader structure"]
impl crate::Readable for MACA2HRrs {}
#[doc = "`write(|w| ..)` method takes [`maca2hr::W`](W) writer structure"]
impl crate::Writable for MACA2HRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA2HR to value 0x50"]
impl crate::Resettable for MACA2HRrs {
    const RESET_VALUE: u32 = 0x50;
}
