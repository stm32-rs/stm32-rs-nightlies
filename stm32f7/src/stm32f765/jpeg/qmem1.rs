#[doc = "Register `QMEM1%s` reader"]
pub type R = crate::R<QMEM1rs>;
#[doc = "Register `QMEM1%s` writer"]
pub type W = crate::W<QMEM1rs>;
#[doc = "Field `QMem_RAM` reader - QMem RAM"]
pub type QMEM_RAM_R = crate::FieldReader<u32>;
#[doc = "Field `QMem_RAM` writer - QMem RAM"]
pub type QMEM_RAM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - QMem RAM"]
    #[inline(always)]
    pub fn qmem_ram(&self) -> QMEM_RAM_R {
        QMEM_RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - QMem RAM"]
    #[inline(always)]
    #[must_use]
    pub fn qmem_ram(&mut self) -> QMEM_RAM_W<QMEM1rs> {
        QMEM_RAM_W::new(self, 0)
    }
}
#[doc = "JPEG quantization tables\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qmem1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qmem1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QMEM1rs;
impl crate::RegisterSpec for QMEM1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qmem1::R`](R) reader structure"]
impl crate::Readable for QMEM1rs {}
#[doc = "`write(|w| ..)` method takes [`qmem1::W`](W) writer structure"]
impl crate::Writable for QMEM1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QMEM1%s to value 0"]
impl crate::Resettable for QMEM1rs {
    const RESET_VALUE: u32 = 0;
}
