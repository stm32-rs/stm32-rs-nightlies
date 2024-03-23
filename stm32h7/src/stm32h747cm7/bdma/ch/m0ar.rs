#[doc = "Register `M0AR` reader"]
pub type R = crate::R<M0ARrs>;
#[doc = "Register `M0AR` writer"]
pub type W = crate::W<M0ARrs>;
#[doc = "Field `MA` reader - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<M0ARrs> {
        MA_W::new(self, 0)
    }
}
#[doc = "This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m0ar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m0ar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M0ARrs;
impl crate::RegisterSpec for M0ARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m0ar::R`](R) reader structure"]
impl crate::Readable for M0ARrs {}
#[doc = "`write(|w| ..)` method takes [`m0ar::W`](W) writer structure"]
impl crate::Writable for M0ARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M0AR to value 0"]
impl crate::Resettable for M0ARrs {
    const RESET_VALUE: u32 = 0;
}
