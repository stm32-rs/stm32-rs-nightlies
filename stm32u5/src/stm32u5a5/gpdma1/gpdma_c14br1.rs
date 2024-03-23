#[doc = "Register `GPDMA_C14BR1` reader"]
pub type R = crate::R<GPDMA_C14BR1rs>;
#[doc = "Register `GPDMA_C14BR1` writer"]
pub type W = crate::W<GPDMA_C14BR1rs>;
#[doc = "Field `BNDT` reader - block number of data bytes to transfer from the source"]
pub type BNDT_R = crate::FieldReader<u16>;
#[doc = "Field `BNDT` writer - block number of data bytes to transfer from the source"]
pub type BNDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BRC` reader - BRC"]
pub type BRC_R = crate::FieldReader<u16>;
#[doc = "Field `BRC` writer - BRC"]
pub type BRC_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SDEC` reader - SDEC"]
pub type SDEC_R = crate::BitReader;
#[doc = "Field `SDEC` writer - SDEC"]
pub type SDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDEC` reader - DDEC"]
pub type DDEC_R = crate::BitReader;
#[doc = "Field `DDEC` writer - DDEC"]
pub type DDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRSDEC` reader - BRSDEC"]
pub type BRSDEC_R = crate::BitReader;
#[doc = "Field `BRSDEC` writer - BRSDEC"]
pub type BRSDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRDDEC` reader - BRDDEC"]
pub type BRDDEC_R = crate::BitReader;
#[doc = "Field `BRDDEC` writer - BRDDEC"]
pub type BRDDEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - block number of data bytes to transfer from the source"]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:26 - BRC"]
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - SDEC"]
    #[inline(always)]
    pub fn sdec(&self) -> SDEC_R {
        SDEC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DDEC"]
    #[inline(always)]
    pub fn ddec(&self) -> DDEC_R {
        DDEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - BRSDEC"]
    #[inline(always)]
    pub fn brsdec(&self) -> BRSDEC_R {
        BRSDEC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - BRDDEC"]
    #[inline(always)]
    pub fn brddec(&self) -> BRDDEC_R {
        BRDDEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - block number of data bytes to transfer from the source"]
    #[inline(always)]
    #[must_use]
    pub fn bndt(&mut self) -> BNDT_W<GPDMA_C14BR1rs> {
        BNDT_W::new(self, 0)
    }
    #[doc = "Bits 16:26 - BRC"]
    #[inline(always)]
    #[must_use]
    pub fn brc(&mut self) -> BRC_W<GPDMA_C14BR1rs> {
        BRC_W::new(self, 16)
    }
    #[doc = "Bit 28 - SDEC"]
    #[inline(always)]
    #[must_use]
    pub fn sdec(&mut self) -> SDEC_W<GPDMA_C14BR1rs> {
        SDEC_W::new(self, 28)
    }
    #[doc = "Bit 29 - DDEC"]
    #[inline(always)]
    #[must_use]
    pub fn ddec(&mut self) -> DDEC_W<GPDMA_C14BR1rs> {
        DDEC_W::new(self, 29)
    }
    #[doc = "Bit 30 - BRSDEC"]
    #[inline(always)]
    #[must_use]
    pub fn brsdec(&mut self) -> BRSDEC_W<GPDMA_C14BR1rs> {
        BRSDEC_W::new(self, 30)
    }
    #[doc = "Bit 31 - BRDDEC"]
    #[inline(always)]
    #[must_use]
    pub fn brddec(&mut self) -> BRDDEC_W<GPDMA_C14BR1rs> {
        BRDDEC_W::new(self, 31)
    }
}
#[doc = "GPDMA channel x block register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdma_c14br1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdma_c14br1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDMA_C14BR1rs;
impl crate::RegisterSpec for GPDMA_C14BR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdma_c14br1::R`](R) reader structure"]
impl crate::Readable for GPDMA_C14BR1rs {}
#[doc = "`write(|w| ..)` method takes [`gpdma_c14br1::W`](W) writer structure"]
impl crate::Writable for GPDMA_C14BR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDMA_C14BR1 to value 0"]
impl crate::Resettable for GPDMA_C14BR1rs {
    const RESET_VALUE: u32 = 0;
}
