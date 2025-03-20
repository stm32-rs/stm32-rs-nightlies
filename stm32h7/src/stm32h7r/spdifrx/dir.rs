///Register `DIR` reader
pub type R = crate::R<DIRrs>;
///Field `THI` reader - threshold HIGH (THI = 2.5 x UI / T<sub>spdifrx_ker_ck</sub>) This field contains the current threshold HIGH estimation. This value can be used to estimate the sampling rate of the received stream. The accuracy of THI is limited to a period of the spdifrx_ker_ck. The sampling rate can be estimated as follow: Sampling Rate = \[2 x THI x T<sub>spdifrx_ker_ck </sub>+/- T<sub>spdifrx_ker_ck</sub>\] x 2/5 Note that THI is updated by the hardware when SYNCD goes high, and then every frame.
pub type THI_R = crate::FieldReader<u16>;
///Field `TLO` reader - threshold LOW (TLO = 1.5 x UI / T<sub>spdifrx_ker_ck</sub>) This field contains the current threshold LOW estimation. This value can be used to estimate the sampling rate of the received stream. The accuracy of TLO is limited to a period of the spdifrx_ker_ck. The sampling rate can be estimated as follow: Sampling Rate = \[2 x TLO x T<sub>spdifrx_ker_ck </sub>+/- T<sub>spdifrx_ker_ck</sub>\] x 2/3 Note that TLO is updated by the hardware when SYNCD goes high, and then every frame.
pub type TLO_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:12 - threshold HIGH (THI = 2.5 x UI / T<sub>spdifrx_ker_ck</sub>) This field contains the current threshold HIGH estimation. This value can be used to estimate the sampling rate of the received stream. The accuracy of THI is limited to a period of the spdifrx_ker_ck. The sampling rate can be estimated as follow: Sampling Rate = \[2 x THI x T<sub>spdifrx_ker_ck </sub>+/- T<sub>spdifrx_ker_ck</sub>\] x 2/5 Note that THI is updated by the hardware when SYNCD goes high, and then every frame.
    #[inline(always)]
    pub fn thi(&self) -> THI_R {
        THI_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - threshold LOW (TLO = 1.5 x UI / T<sub>spdifrx_ker_ck</sub>) This field contains the current threshold LOW estimation. This value can be used to estimate the sampling rate of the received stream. The accuracy of TLO is limited to a period of the spdifrx_ker_ck. The sampling rate can be estimated as follow: Sampling Rate = \[2 x TLO x T<sub>spdifrx_ker_ck </sub>+/- T<sub>spdifrx_ker_ck</sub>\] x 2/3 Note that TLO is updated by the hardware when SYNCD goes high, and then every frame.
    #[inline(always)]
    pub fn tlo(&self) -> TLO_R {
        TLO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIR")
            .field("thi", &self.thi())
            .field("tlo", &self.tlo())
            .finish()
    }
}
/**SPDIFRX debug information register

You can [`read`](crate::Reg::read) this register and get [`dir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SPDIFRX:DIR)*/
pub struct DIRrs;
impl crate::RegisterSpec for DIRrs {
    type Ux = u32;
}
///`read()` method returns [`dir::R`](R) reader structure
impl crate::Readable for DIRrs {}
///`reset()` method sets DIR to value 0
impl crate::Resettable for DIRrs {}
