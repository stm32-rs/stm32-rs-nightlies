///Register `BPR` reader
pub type R = crate::R<BPRrs>;
///Register `BPR` writer
pub type W = crate::W<BPRrs>;
///Field `BINARY_POINT` reader - BINARY_POINT
pub type BINARY_POINT_R = crate::FieldReader;
///Field `BINARY_POINT` writer - BINARY_POINT
pub type BINARY_POINT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - BINARY_POINT
    #[inline(always)]
    pub fn binary_point(&self) -> BINARY_POINT_R {
        BINARY_POINT_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPR")
            .field("binary_point", &self.binary_point())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - BINARY_POINT
    #[inline(always)]
    pub fn binary_point(&mut self) -> BINARY_POINT_W<'_, BPRrs> {
        BINARY_POINT_W::new(self, 0)
    }
}
/**GICV VM binary point register

You can [`read`](crate::Reg::read) this register and get [`bpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICV:BPR)*/
pub struct BPRrs;
impl crate::RegisterSpec for BPRrs {
    type Ux = u32;
}
///`read()` method returns [`bpr::R`](R) reader structure
impl crate::Readable for BPRrs {}
///`write(|w| ..)` method takes [`bpr::W`](W) writer structure
impl crate::Writable for BPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BPR to value 0x02
impl crate::Resettable for BPRrs {
    const RESET_VALUE: u32 = 0x02;
}
