#[doc = "Register `HDPEXTR` reader"]
pub type R = crate::R<HDPEXTRrs>;
#[doc = "Register `HDPEXTR` writer"]
pub type W = crate::W<HDPEXTRrs>;
#[doc = "Field `HDP1_EXT` reader - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
pub type HDP1_EXT_R = crate::FieldReader;
#[doc = "Field `HDP1_EXT` writer - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
pub type HDP1_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HDP2_EXT` reader - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
pub type HDP2_EXT_R = crate::FieldReader;
#[doc = "Field `HDP2_EXT` writer - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
pub type HDP2_EXT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
    #[inline(always)]
    pub fn hdp1_ext(&self) -> HDP1_EXT_R {
        HDP1_EXT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
    #[inline(always)]
    pub fn hdp2_ext(&self) -> HDP2_EXT_R {
        HDP2_EXT_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
    #[inline(always)]
    #[must_use]
    pub fn hdp1_ext(&mut self) -> HDP1_EXT_W<HDPEXTRrs> {
        HDP1_EXT_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
    #[inline(always)]
    #[must_use]
    pub fn hdp2_ext(&mut self) -> HDP2_EXT_W<HDPEXTRrs> {
        HDP2_EXT_W::new(self, 16)
    }
}
#[doc = "FLASH HDP extension register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdpextr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdpextr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDPEXTRrs;
impl crate::RegisterSpec for HDPEXTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdpextr::R`](R) reader structure"]
impl crate::Readable for HDPEXTRrs {}
#[doc = "`write(|w| ..)` method takes [`hdpextr::W`](W) writer structure"]
impl crate::Writable for HDPEXTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDPEXTR to value 0"]
impl crate::Resettable for HDPEXTRrs {
    const RESET_VALUE: u32 = 0;
}
