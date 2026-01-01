///Register `BRR` reader
pub type R = crate::R<BRRrs>;
///Register `BRR` writer
pub type W = crate::W<BRRrs>;
///Field `BRR_0_3` reader - BRR_0_3
pub type BRR_0_3_R = crate::FieldReader;
///Field `BRR_0_3` writer - BRR_0_3
pub type BRR_0_3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BRR_4_15` reader - BRR_4_15
pub type BRR_4_15_R = crate::FieldReader<u16>;
///Field `BRR_4_15` writer - BRR_4_15
pub type BRR_4_15_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:3 - BRR_0_3
    #[inline(always)]
    pub fn brr_0_3(&self) -> BRR_0_3_R {
        BRR_0_3_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:15 - BRR_4_15
    #[inline(always)]
    pub fn brr_4_15(&self) -> BRR_4_15_R {
        BRR_4_15_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRR")
            .field("brr_4_15", &self.brr_4_15())
            .field("brr_0_3", &self.brr_0_3())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - BRR_0_3
    #[inline(always)]
    pub fn brr_0_3(&mut self) -> BRR_0_3_W<'_, BRRrs> {
        BRR_0_3_W::new(self, 0)
    }
    ///Bits 4:15 - BRR_4_15
    #[inline(always)]
    pub fn brr_4_15(&mut self) -> BRR_4_15_W<'_, BRRrs> {
        BRR_4_15_W::new(self, 4)
    }
}
/**Baud rate register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#USART1:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRRrs {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {}
