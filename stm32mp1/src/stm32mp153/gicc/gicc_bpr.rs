///Register `GICC_BPR` reader
pub type R = crate::R<GICC_BPRrs>;
///Register `GICC_BPR` writer
pub type W = crate::W<GICC_BPRrs>;
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
        f.debug_struct("GICC_BPR")
            .field("binary_point", &self.binary_point())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - BINARY_POINT
    #[inline(always)]
    #[must_use]
    pub fn binary_point(&mut self) -> BINARY_POINT_W<GICC_BPRrs> {
        BINARY_POINT_W::new(self, 0)
    }
}
/**GICC binary point register

You can [`read`](crate::Reg::read) this register and get [`gicc_bpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicc_bpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICC:GICC_BPR)*/
pub struct GICC_BPRrs;
impl crate::RegisterSpec for GICC_BPRrs {
    type Ux = u32;
}
///`read()` method returns [`gicc_bpr::R`](R) reader structure
impl crate::Readable for GICC_BPRrs {}
///`write(|w| ..)` method takes [`gicc_bpr::W`](W) writer structure
impl crate::Writable for GICC_BPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICC_BPR to value 0x02
impl crate::Resettable for GICC_BPRrs {
    const RESET_VALUE: u32 = 0x02;
}
