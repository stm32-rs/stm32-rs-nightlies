///Register `TIMINGR2` reader
pub type R = crate::R<TIMINGR2rs>;
///Register `TIMINGR2` writer
pub type W = crate::W<TIMINGR2rs>;
///Field `STALLT` reader - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent.
pub type STALLT_R = crate::BitReader;
///Field `STALLT` writer - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent.
pub type STALLT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALLD` reader - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data.
pub type STALLD_R = crate::BitReader;
///Field `STALLD` writer - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data.
pub type STALLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALLC` reader - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command.
pub type STALLC_R = crate::BitReader;
///Field `STALLC` writer - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command.
pub type STALLC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALLA` reader - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.
pub type STALLA_R = crate::BitReader;
///Field `STALLA` writer - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.
pub type STALLA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALL` reader - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK
pub type STALL_R = crate::FieldReader;
///Field `STALL` writer - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK
pub type STALL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent.
    #[inline(always)]
    pub fn stallt(&self) -> STALLT_R {
        STALLT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data.
    #[inline(always)]
    pub fn stalld(&self) -> STALLD_R {
        STALLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command.
    #[inline(always)]
    pub fn stallc(&self) -> STALLC_R {
        STALLC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.
    #[inline(always)]
    pub fn stalla(&self) -> STALLA_R {
        STALLA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:15 - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMINGR2")
            .field("stallt", &self.stallt())
            .field("stalld", &self.stalld())
            .field("stallc", &self.stallc())
            .field("stalla", &self.stalla())
            .field("stall", &self.stall())
            .finish()
    }
}
impl W {
    ///Bit 0 - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent.
    #[inline(always)]
    pub fn stallt(&mut self) -> STALLT_W<'_, TIMINGR2rs> {
        STALLT_W::new(self, 0)
    }
    ///Bit 1 - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data.
    #[inline(always)]
    pub fn stalld(&mut self) -> STALLD_W<'_, TIMINGR2rs> {
        STALLD_W::new(self, 1)
    }
    ///Bit 2 - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command.
    #[inline(always)]
    pub fn stallc(&mut self) -> STALLC_W<'_, TIMINGR2rs> {
        STALLC_W::new(self, 2)
    }
    ///Bit 3 - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.
    #[inline(always)]
    pub fn stalla(&mut self) -> STALLA_W<'_, TIMINGR2rs> {
        STALLA_W::new(self, 3)
    }
    ///Bits 8:15 - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, TIMINGR2rs> {
        STALL_W::new(self, 8)
    }
}
/**I3C timing register 2

You can [`read`](crate::Reg::read) this register and get [`timingr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:TIMINGR2)*/
pub struct TIMINGR2rs;
impl crate::RegisterSpec for TIMINGR2rs {
    type Ux = u32;
}
///`read()` method returns [`timingr2::R`](R) reader structure
impl crate::Readable for TIMINGR2rs {}
///`write(|w| ..)` method takes [`timingr2::W`](W) writer structure
impl crate::Writable for TIMINGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMINGR2 to value 0
impl crate::Resettable for TIMINGR2rs {}
