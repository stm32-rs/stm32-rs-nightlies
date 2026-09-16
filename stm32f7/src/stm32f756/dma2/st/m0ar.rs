///Register `M0AR` reader
pub type R = crate::R<M0ARrs>;
///Register `M0AR` writer
pub type W = crate::W<M0ARrs>;
///Field `M0A` reader - Memory 0 address
pub type M0A_R = crate::FieldReader<u32>;
///Field `M0A` writer - Memory 0 address
pub type M0A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory 0 address
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M0AR").field("m0a", &self.m0a()).finish()
    }
}
impl W {
    ///Bits 0:31 - Memory 0 address
    #[inline(always)]
    pub fn m0a(&mut self) -> M0A_W<'_, M0ARrs> {
        M0A_W::new(self, 0)
    }
}
/**stream x memory 0 address register

You can [`read`](crate::Reg::read) this register and get [`m0ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m0ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct M0ARrs;
impl crate::RegisterSpec for M0ARrs {
    type Ux = u32;
}
///`read()` method returns [`m0ar::R`](R) reader structure
impl crate::Readable for M0ARrs {}
///`write(|w| ..)` method takes [`m0ar::W`](W) writer structure
impl crate::Writable for M0ARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M0AR to value 0
impl crate::Resettable for M0ARrs {}
