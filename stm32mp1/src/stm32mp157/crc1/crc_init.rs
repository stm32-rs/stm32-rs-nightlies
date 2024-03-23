#[doc = "Register `CRC_INIT` reader"]
pub type R = crate::R<CRC_INITrs>;
#[doc = "Register `CRC_INIT` writer"]
pub type W = crate::W<CRC_INITrs>;
#[doc = "Field `CRC_INIT` reader - CRC_INIT"]
pub type CRC_INIT_R = crate::FieldReader<u32>;
#[doc = "Field `CRC_INIT` writer - CRC_INIT"]
pub type CRC_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC_INIT"]
    #[inline(always)]
    pub fn crc_init(&self) -> CRC_INIT_R {
        CRC_INIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC_INIT"]
    #[inline(always)]
    #[must_use]
    pub fn crc_init(&mut self) -> CRC_INIT_W<CRC_INITrs> {
        CRC_INIT_W::new(self, 0)
    }
}
#[doc = "CRC initial value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRC_INITrs;
impl crate::RegisterSpec for CRC_INITrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_init::R`](R) reader structure"]
impl crate::Readable for CRC_INITrs {}
#[doc = "`write(|w| ..)` method takes [`crc_init::W`](W) writer structure"]
impl crate::Writable for CRC_INITrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_INIT to value 0xffff_ffff"]
impl crate::Resettable for CRC_INITrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
