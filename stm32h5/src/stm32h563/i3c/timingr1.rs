///Register `TIMINGR1` reader
pub type R = crate::R<TIMINGR1rs>;
///Register `TIMINGR1` writer
pub type W = crate::W<TIMINGR1rs>;
/**Field `AVAL` reader - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 s. This timing is defined by: tAVAL = (AVAL\[7:0\]
+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 s . This timing is defined by: tIDLE = (AVAL\[7:0\]
+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\[7:0\]
+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 s. This timing is defined by: tSTALL = (AVAL\[7:0\]
+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\[1:0\], as follows: If ASNCR\[1:0\]=00: tNEWCRLock = (AVAL\[7:0\]
+ 1) x tI3CCLK If ASNCR\[1:0\]=01: tNEWCRLock = (AVAL\[7:0\]
+ 1) x 100 x tI3CCLK If ASNCR\[1:0\]=10: tNEWCRLock = (AVAL\[7:0\]
+ 1) x 2000 x tI3CCLK If ASNCR\[1:0\]=11: tNEWCRLock = (AVAL\[7:0\]
+ 1) x 50000 x tI3CCLK*/
pub type AVAL_R = crate::FieldReader;
/**Field `AVAL` writer - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 s. This timing is defined by: tAVAL = (AVAL\[7:0\]
+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 s . This timing is defined by: tIDLE = (AVAL\[7:0\]
+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\[7:0\]
+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 s. This timing is defined by: tSTALL = (AVAL\[7:0\]
+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\[1:0\], as follows: If ASNCR\[1:0\]=00: tNEWCRLock = (AVAL\[7:0\]
+ 1) x tI3CCLK If ASNCR\[1:0\]=01: tNEWCRLock = (AVAL\[7:0\]
+ 1) x 100 x tI3CCLK If ASNCR\[1:0\]=10: tNEWCRLock = (AVAL\[7:0\]
+ 1) x 2000 x tI3CCLK If ASNCR\[1:0\]=11: tNEWCRLock = (AVAL\[7:0\]
+ 1) x 50000 x tI3CCLK*/
pub type AVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ASNCR` reader - activity state of the new controller (when I3C is acting asactive- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\[7:0\]. This field can be modified only when the I3C is acting as controller.
pub type ASNCR_R = crate::FieldReader;
///Field `ASNCR` writer - activity state of the new controller (when I3C is acting asactive- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\[7:0\]. This field can be modified only when the I3C is acting as controller.
pub type ASNCR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Field `FREE` reader - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \[ (FREE\[6:0\]
+ 1) x 2(0,5 + SDA_HD)\]
x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1s, 100s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \[ (FREE\[6:0\]
+ 1) x 2(0,5 + SDA_HD)\]
x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\[6:0\]
+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\[6:0\]
+ 1) x tI3CCLK*/
pub type FREE_R = crate::FieldReader;
/**Field `FREE` writer - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \[ (FREE\[6:0\]
+ 1) x 2(0,5 + SDA_HD)\]
x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1s, 100s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \[ (FREE\[6:0\]
+ 1) x 2(0,5 + SDA_HD)\]
x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\[6:0\]
+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\[6:0\]
+ 1) x tI3CCLK*/
pub type FREE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SDA_HD` reader - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):
pub type SDA_HD_R = crate::BitReader;
///Field `SDA_HD` writer - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):
pub type SDA_HD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bits 0:7 - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 s. This timing is defined by: tAVAL = (AVAL\[7:0\]
    + 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 s . This timing is defined by: tIDLE = (AVAL\[7:0\]
    + 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\[7:0\]
    + 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 s. This timing is defined by: tSTALL = (AVAL\[7:0\]
    + 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\[1:0\], as follows: If ASNCR\[1:0\]=00: tNEWCRLock = (AVAL\[7:0\]
    + 1) x tI3CCLK If ASNCR\[1:0\]=01: tNEWCRLock = (AVAL\[7:0\]
    + 1) x 100 x tI3CCLK If ASNCR\[1:0\]=10: tNEWCRLock = (AVAL\[7:0\]
    + 1) x 2000 x tI3CCLK If ASNCR\[1:0\]=11: tNEWCRLock = (AVAL\[7:0\]
    + 1) x 50000 x tI3CCLK*/
    #[inline(always)]
    pub fn aval(&self) -> AVAL_R {
        AVAL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - activity state of the new controller (when I3C is acting asactive- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\[7:0\]. This field can be modified only when the I3C is acting as controller.
    #[inline(always)]
    pub fn asncr(&self) -> ASNCR_R {
        ASNCR_R::new(((self.bits >> 8) & 3) as u8)
    }
    /**Bits 16:22 - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \[ (FREE\[6:0\]
    + 1) x 2(0,5 + SDA_HD)\]
    x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1s, 100s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \[ (FREE\[6:0\]
    + 1) x 2(0,5 + SDA_HD)\]
    x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\[6:0\]
    + 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\[6:0\]
    + 1) x tI3CCLK*/
    #[inline(always)]
    pub fn free(&self) -> FREE_R {
        FREE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 28 - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):
    #[inline(always)]
    pub fn sda_hd(&self) -> SDA_HD_R {
        SDA_HD_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMINGR1")
            .field("aval", &self.aval())
            .field("asncr", &self.asncr())
            .field("free", &self.free())
            .field("sda_hd", &self.sda_hd())
            .finish()
    }
}
impl W {
    /**Bits 0:7 - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 s. This timing is defined by: tAVAL = (AVAL\[7:0\]
    + 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 s . This timing is defined by: tIDLE = (AVAL\[7:0\]
    + 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\[7:0\]
    + 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 s. This timing is defined by: tSTALL = (AVAL\[7:0\]
    + 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\[1:0\], as follows: If ASNCR\[1:0\]=00: tNEWCRLock = (AVAL\[7:0\]
    + 1) x tI3CCLK If ASNCR\[1:0\]=01: tNEWCRLock = (AVAL\[7:0\]
    + 1) x 100 x tI3CCLK If ASNCR\[1:0\]=10: tNEWCRLock = (AVAL\[7:0\]
    + 1) x 2000 x tI3CCLK If ASNCR\[1:0\]=11: tNEWCRLock = (AVAL\[7:0\]
    + 1) x 50000 x tI3CCLK*/
    #[inline(always)]
    pub fn aval(&mut self) -> AVAL_W<TIMINGR1rs> {
        AVAL_W::new(self, 0)
    }
    ///Bits 8:9 - activity state of the new controller (when I3C is acting asactive- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\[7:0\]. This field can be modified only when the I3C is acting as controller.
    #[inline(always)]
    pub fn asncr(&mut self) -> ASNCR_W<TIMINGR1rs> {
        ASNCR_W::new(self, 8)
    }
    /**Bits 16:22 - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \[ (FREE\[6:0\]
    + 1) x 2(0,5 + SDA_HD)\]
    x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1s, 100s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \[ (FREE\[6:0\]
    + 1) x 2(0,5 + SDA_HD)\]
    x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\[6:0\]
    + 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\[6:0\]
    + 1) x tI3CCLK*/
    #[inline(always)]
    pub fn free(&mut self) -> FREE_W<TIMINGR1rs> {
        FREE_W::new(self, 16)
    }
    ///Bit 28 - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):
    #[inline(always)]
    pub fn sda_hd(&mut self) -> SDA_HD_W<TIMINGR1rs> {
        SDA_HD_W::new(self, 28)
    }
}
/**I3C timing register 1

You can [`read`](crate::Reg::read) this register and get [`timingr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#I3C:TIMINGR1)*/
pub struct TIMINGR1rs;
impl crate::RegisterSpec for TIMINGR1rs {
    type Ux = u32;
}
///`read()` method returns [`timingr1::R`](R) reader structure
impl crate::Readable for TIMINGR1rs {}
///`write(|w| ..)` method takes [`timingr1::W`](W) writer structure
impl crate::Writable for TIMINGR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMINGR1 to value 0
impl crate::Resettable for TIMINGR1rs {
    const RESET_VALUE: u32 = 0;
}