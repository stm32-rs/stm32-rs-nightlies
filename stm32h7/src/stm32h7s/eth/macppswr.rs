///Register `MACPPSWR` reader
pub type R = crate::R<MACPPSWRrs>;
///Register `MACPPSWR` writer
pub type W = crate::W<MACPPSWRrs>;
///Field `PPSWIDTH0` reader - PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS signal output. The width is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20 ns), and width between the rising and corresponding falling edges of PPS signal output is 80 ns (that is, four units of subsecond increment value), you should program value 3 (4-1) in this register. Note: The value programmed in this register must be lesser than the value programmed in PPS interval register (ETH_MACPPSIR).
pub type PPSWIDTH0_R = crate::FieldReader<u32>;
///Field `PPSWIDTH0` writer - PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS signal output. The width is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20 ns), and width between the rising and corresponding falling edges of PPS signal output is 80 ns (that is, four units of subsecond increment value), you should program value 3 (4-1) in this register. Note: The value programmed in this register must be lesser than the value programmed in PPS interval register (ETH_MACPPSIR).
pub type PPSWIDTH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS signal output. The width is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20 ns), and width between the rising and corresponding falling edges of PPS signal output is 80 ns (that is, four units of subsecond increment value), you should program value 3 (4-1) in this register. Note: The value programmed in this register must be lesser than the value programmed in PPS interval register (ETH_MACPPSIR).
    #[inline(always)]
    pub fn ppswidth0(&self) -> PPSWIDTH0_R {
        PPSWIDTH0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSWR")
            .field("ppswidth0", &self.ppswidth0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS signal output. The width is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20 ns), and width between the rising and corresponding falling edges of PPS signal output is 80 ns (that is, four units of subsecond increment value), you should program value 3 (4-1) in this register. Note: The value programmed in this register must be lesser than the value programmed in PPS interval register (ETH_MACPPSIR).
    #[inline(always)]
    pub fn ppswidth0(&mut self) -> PPSWIDTH0_W<MACPPSWRrs> {
        PPSWIDTH0_W::new(self, 0)
    }
}
/**PPS width register

You can [`read`](crate::Reg::read) this register and get [`macppswr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppswr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#ETH:MACPPSWR)*/
pub struct MACPPSWRrs;
impl crate::RegisterSpec for MACPPSWRrs {
    type Ux = u32;
}
///`read()` method returns [`macppswr::R`](R) reader structure
impl crate::Readable for MACPPSWRrs {}
///`write(|w| ..)` method takes [`macppswr::W`](W) writer structure
impl crate::Writable for MACPPSWRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSWR to value 0
impl crate::Resettable for MACPPSWRrs {}
