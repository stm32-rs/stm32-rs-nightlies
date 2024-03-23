#[doc = "Register `MACPPSWR` reader"]
pub type R = crate::R<MACPPSWRrs>;
#[doc = "Register `MACPPSWR` writer"]
pub type W = crate::W<MACPPSWRrs>;
#[doc = "Field `PPSWIDTH0` reader - PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS signal output. The width is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20 ns), and width between the rising and corresponding falling edges of PPS signal output is 80 ns (that is, four units of subsecond increment value), you should program value 3 (4-1) in this register. Note: The value programmed in this register must be lesser than the value programmed in PPS interval register (ETH_MACPPSIR)."]
pub type PPSWIDTH0_R = crate::FieldReader<u32>;
#[doc = "Field `PPSWIDTH0` writer - PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS signal output. The width is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20 ns), and width between the rising and corresponding falling edges of PPS signal output is 80 ns (that is, four units of subsecond increment value), you should program value 3 (4-1) in this register. Note: The value programmed in this register must be lesser than the value programmed in PPS interval register (ETH_MACPPSIR)."]
pub type PPSWIDTH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS signal output. The width is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20 ns), and width between the rising and corresponding falling edges of PPS signal output is 80 ns (that is, four units of subsecond increment value), you should program value 3 (4-1) in this register. Note: The value programmed in this register must be lesser than the value programmed in PPS interval register (ETH_MACPPSIR)."]
    #[inline(always)]
    pub fn ppswidth0(&self) -> PPSWIDTH0_R {
        PPSWIDTH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS Output Signal Width These bits store the width between the rising edge and corresponding falling edge of PPS signal output. The width is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20 ns), and width between the rising and corresponding falling edges of PPS signal output is 80 ns (that is, four units of subsecond increment value), you should program value 3 (4-1) in this register. Note: The value programmed in this register must be lesser than the value programmed in PPS interval register (ETH_MACPPSIR)."]
    #[inline(always)]
    #[must_use]
    pub fn ppswidth0(&mut self) -> PPSWIDTH0_W<MACPPSWRrs> {
        PPSWIDTH0_W::new(self, 0)
    }
}
#[doc = "PPS width register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppswr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppswr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPPSWRrs;
impl crate::RegisterSpec for MACPPSWRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppswr::R`](R) reader structure"]
impl crate::Readable for MACPPSWRrs {}
#[doc = "`write(|w| ..)` method takes [`macppswr::W`](W) writer structure"]
impl crate::Writable for MACPPSWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPPSWR to value 0"]
impl crate::Resettable for MACPPSWRrs {
    const RESET_VALUE: u32 = 0;
}
