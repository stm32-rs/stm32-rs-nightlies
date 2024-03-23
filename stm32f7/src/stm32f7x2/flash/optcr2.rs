#[doc = "Register `OPTCR2` reader"]
pub type R = crate::R<OPTCR2rs>;
#[doc = "Register `OPTCR2` writer"]
pub type W = crate::W<OPTCR2rs>;
#[doc = "Field `PCROPi` reader - PCROP option byte"]
pub type PCROPI_R = crate::FieldReader;
#[doc = "Field `PCROPi` writer - PCROP option byte"]
pub type PCROPI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PCROP_RDP` reader - PCROP zone preserved when RDP level decreased"]
pub type PCROP_RDP_R = crate::BitReader;
#[doc = "Field `PCROP_RDP` writer - PCROP zone preserved when RDP level decreased"]
pub type PCROP_RDP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PCROP option byte"]
    #[inline(always)]
    pub fn pcropi(&self) -> PCROPI_R {
        PCROPI_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - PCROP zone preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PCROP option byte"]
    #[inline(always)]
    #[must_use]
    pub fn pcropi(&mut self) -> PCROPI_W<OPTCR2rs> {
        PCROPI_W::new(self, 0)
    }
    #[doc = "Bit 31 - PCROP zone preserved when RDP level decreased"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<OPTCR2rs> {
        PCROP_RDP_W::new(self, 31)
    }
}
#[doc = "Flash option control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTCR2rs;
impl crate::RegisterSpec for OPTCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optcr2::R`](R) reader structure"]
impl crate::Readable for OPTCR2rs {}
#[doc = "`write(|w| ..)` method takes [`optcr2::W`](W) writer structure"]
impl crate::Writable for OPTCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTCR2 to value 0x8000_00ff"]
impl crate::Resettable for OPTCR2rs {
    const RESET_VALUE: u32 = 0x8000_00ff;
}
