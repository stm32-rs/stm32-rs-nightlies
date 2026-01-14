///Register `DERATEINT` reader
pub type R = crate::R<DERATEINTrs>;
///Register `DERATEINT` writer
pub type W = crate::W<DERATEINTrs>;
///Field `MR4_READ_INTERVAL` reader - MR4_READ_INTERVAL
pub type MR4_READ_INTERVAL_R = crate::FieldReader<u32>;
///Field `MR4_READ_INTERVAL` writer - MR4_READ_INTERVAL
pub type MR4_READ_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - MR4_READ_INTERVAL
    #[inline(always)]
    pub fn mr4_read_interval(&self) -> MR4_READ_INTERVAL_R {
        MR4_READ_INTERVAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DERATEINT")
            .field("mr4_read_interval", &self.mr4_read_interval())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - MR4_READ_INTERVAL
    #[inline(always)]
    pub fn mr4_read_interval(&mut self) -> MR4_READ_INTERVAL_W<'_, DERATEINTrs> {
        MR4_READ_INTERVAL_W::new(self, 0)
    }
}
/**DDRCTRL temperature derate interval register

You can [`read`](crate::Reg::read) this register and get [`derateint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`derateint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DERATEINT)*/
pub struct DERATEINTrs;
impl crate::RegisterSpec for DERATEINTrs {
    type Ux = u32;
}
///`read()` method returns [`derateint::R`](R) reader structure
impl crate::Readable for DERATEINTrs {}
///`write(|w| ..)` method takes [`derateint::W`](W) writer structure
impl crate::Writable for DERATEINTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DERATEINT to value 0x0080_0000
impl crate::Resettable for DERATEINTrs {
    const RESET_VALUE: u32 = 0x0080_0000;
}
