#[doc = "Register `CRC_POL` reader"]
pub type R = crate::R<CRC_POLrs>;
#[doc = "Register `CRC_POL` writer"]
pub type W = crate::W<CRC_POLrs>;
#[doc = "Field `POL` reader - POL"]
pub type POL_R = crate::FieldReader<u32>;
#[doc = "Field `POL` writer - POL"]
pub type POL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - POL"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CRC_POLrs> {
        POL_W::new(self, 0)
    }
}
#[doc = "CRC polynomial\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_pol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_pol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_POLrs;
impl crate::RegisterSpec for CRC_POLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_pol::R`](R) reader structure"]
impl crate::Readable for CRC_POLrs {}
#[doc = "`write(|w| ..)` method takes [`crc_pol::W`](W) writer structure"]
impl crate::Writable for CRC_POLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_POL to value 0x04c1_1db7"]
impl crate::Resettable for CRC_POLrs {
    const RESET_VALUE: u32 = 0x04c1_1db7;
}
