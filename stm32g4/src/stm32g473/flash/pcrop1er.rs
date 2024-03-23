#[doc = "Register `PCROP1ER` reader"]
pub type R = crate::R<PCROP1ERrs>;
#[doc = "Register `PCROP1ER` writer"]
pub type W = crate::W<PCROP1ERrs>;
#[doc = "Field `PCROP1_END` reader - Bank 1 PCROP area end offset"]
pub type PCROP1_END_R = crate::FieldReader<u16>;
#[doc = "Field `PCROP1_END` writer - Bank 1 PCROP area end offset"]
pub type PCROP1_END_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_R = crate::BitReader;
#[doc = "Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1_end(&self) -> PCROP1_END_R {
        PCROP1_END_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1_end(&mut self) -> PCROP1_END_W<PCROP1ERrs> {
        PCROP1_END_W::new(self, 0)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<PCROP1ERrs> {
        PCROP_RDP_W::new(self, 31)
    }
}
#[doc = "Flash Bank 1 PCROP End address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1er::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1er::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP1ERrs;
impl crate::RegisterSpec for PCROP1ERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1er::R`](R) reader structure"]
impl crate::Readable for PCROP1ERrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop1er::W`](W) writer structure"]
impl crate::Writable for PCROP1ERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP1ER to value 0x0fff_0000"]
impl crate::Resettable for PCROP1ERrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
