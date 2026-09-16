///Register `MACPPSTTSR` reader
pub type R = crate::R<MACPPSTTSRrs>;
///Register `MACPPSTTSR` writer
pub type W = crate::W<MACPPSTTSRrs>;
///Field `TSTRH0` reader - PPS Target Time Seconds Register This field stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on Target Time mode selected for the corresponding PPS output in the PPS control register (ETH_MACPPSCR).
pub type TSTRH0_R = crate::FieldReader<u32>;
///Field `TSTRH0` writer - PPS Target Time Seconds Register This field stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on Target Time mode selected for the corresponding PPS output in the PPS control register (ETH_MACPPSCR).
pub type TSTRH0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - PPS Target Time Seconds Register This field stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on Target Time mode selected for the corresponding PPS output in the PPS control register (ETH_MACPPSCR).
    #[inline(always)]
    pub fn tstrh0(&self) -> TSTRH0_R {
        TSTRH0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPPSTTSR")
            .field("tstrh0", &self.tstrh0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - PPS Target Time Seconds Register This field stores the time in seconds. When the timestamp value matches or exceeds both Target Timestamp registers, the MAC starts or stops the PPS signal output and generates an interrupt (if enabled) based on Target Time mode selected for the corresponding PPS output in the PPS control register (ETH_MACPPSCR).
    #[inline(always)]
    pub fn tstrh0(&mut self) -> TSTRH0_W<'_, MACPPSTTSRrs> {
        TSTRH0_W::new(self, 0)
    }
}
/**PPS target time seconds register

You can [`read`](crate::Reg::read) this register and get [`macppsttsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macppsttsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ETH:MACPPSTTSR)*/
pub struct MACPPSTTSRrs;
impl crate::RegisterSpec for MACPPSTTSRrs {
    type Ux = u32;
}
///`read()` method returns [`macppsttsr::R`](R) reader structure
impl crate::Readable for MACPPSTTSRrs {}
///`write(|w| ..)` method takes [`macppsttsr::W`](W) writer structure
impl crate::Writable for MACPPSTTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACPPSTTSR to value 0
impl crate::Resettable for MACPPSTTSRrs {}
