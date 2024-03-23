#[doc = "Register `ETH_MACA2HR` reader"]
pub type R = crate::R<ETH_MACA2HRrs>;
#[doc = "Register `ETH_MACA2HR` writer"]
pub type W = crate::W<ETH_MACA2HRrs>;
#[doc = "Field `ADDRHI` reader - ADDRHI"]
pub type ADDRHI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - ADDRHI"]
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - MBC"]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - MBC"]
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - SA"]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - SA"]
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - AE"]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - AE"]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    #[must_use]
    pub fn addrhi(&mut self) -> ADDRHI_W<ETH_MACA2HRrs> {
        ADDRHI_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<ETH_MACA2HRrs> {
        MBC_W::new(self, 24)
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<ETH_MACA2HRrs> {
        SA_W::new(self, 30)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<ETH_MACA2HRrs> {
        AE_W::new(self, 31)
    }
}
#[doc = "The MAC Address x High register holds the upper 16 bits of the second 6-byte MAC address of the station.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_maca2hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_maca2hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACA2HRrs;
impl crate::RegisterSpec for ETH_MACA2HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_maca2hr::R`](R) reader structure"]
impl crate::Readable for ETH_MACA2HRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_maca2hr::W`](W) writer structure"]
impl crate::Writable for ETH_MACA2HRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACA2HR to value 0xffff"]
impl crate::Resettable for ETH_MACA2HRrs {
    const RESET_VALUE: u32 = 0xffff;
}
