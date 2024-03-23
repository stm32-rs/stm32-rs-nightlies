#[doc = "Register `HUFFBASE%s` reader"]
pub type R = crate::R<HUFFBASErs>;
#[doc = "Register `HUFFBASE%s` writer"]
pub type W = crate::W<HUFFBASErs>;
#[doc = "Field `HuffBase_RAM_0` reader - HuffBase RAM"]
pub type HUFF_BASE_RAM_0_R = crate::FieldReader<u16>;
#[doc = "Field `HuffBase_RAM_0` writer - HuffBase RAM"]
pub type HUFF_BASE_RAM_0_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `HuffBase_RAM_1` reader - HuffBase RAM"]
pub type HUFF_BASE_RAM_1_R = crate::FieldReader<u16>;
#[doc = "Field `HuffBase_RAM_1` writer - HuffBase RAM"]
pub type HUFF_BASE_RAM_1_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - HuffBase RAM"]
    #[inline(always)]
    pub fn huff_base_ram_0(&self) -> HUFF_BASE_RAM_0_R {
        HUFF_BASE_RAM_0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - HuffBase RAM"]
    #[inline(always)]
    pub fn huff_base_ram_1(&self) -> HUFF_BASE_RAM_1_R {
        HUFF_BASE_RAM_1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - HuffBase RAM"]
    #[inline(always)]
    #[must_use]
    pub fn huff_base_ram_0(&mut self) -> HUFF_BASE_RAM_0_W<HUFFBASErs> {
        HUFF_BASE_RAM_0_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - HuffBase RAM"]
    #[inline(always)]
    #[must_use]
    pub fn huff_base_ram_1(&mut self) -> HUFF_BASE_RAM_1_W<HUFFBASErs> {
        HUFF_BASE_RAM_1_W::new(self, 16)
    }
}
#[doc = "JPEG HuffSymb tables\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`huffbase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`huffbase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUFFBASErs;
impl crate::RegisterSpec for HUFFBASErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`huffbase::R`](R) reader structure"]
impl crate::Readable for HUFFBASErs {}
#[doc = "`write(|w| ..)` method takes [`huffbase::W`](W) writer structure"]
impl crate::Writable for HUFFBASErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HUFFBASE%s to value 0"]
impl crate::Resettable for HUFFBASErs {
    const RESET_VALUE: u32 = 0;
}
