///Register `FCR0` writer
pub type W = crate::W<FCR0rs>;
///Field `CLB0F` writer - Clear line/byte counter 0 flag
pub type CLB0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLB1F` writer - Clear line/byte counter 1 flag
pub type CLB1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLB2F` writer - Clear line/byte counter 2 flag
pub type CLB2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLB3F` writer - Clear line/byte counter 3 flag
pub type CLB3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM0F` writer - Clear timer 0 flag
pub type CTIM0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM1F` writer - Clear timer 1 flag
pub type CTIM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM2F` writer - Clear timer 2 flag
pub type CTIM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIM3F` writer - Clear timer 3 flag
pub type CTIM3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF0F` writer - Clear SOF flag for virtual channel 0
pub type CSOF0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF1F` writer - Clear SOF flag for virtual channel 1
pub type CSOF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF2F` writer - Clear SOF flag for virtual channel 2
pub type CSOF2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSOF3F` writer - Clear SOF flag for virtual channel 3
pub type CSOF3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEOF0F` writer - Clear EOF flag for virtual channel 0
pub type CEOF0F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEOF1F` writer - Clear EOF flag for virtual channel 1
pub type CEOF1F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEOF2F` writer - Clear EOF flag for virtual channel 2
pub type CEOF2F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CEOF3F` writer - Clear EOF flag for virtual channel 3
pub type CEOF3F_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPKTF` writer - Clear short packet flag
pub type CSPKTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCCFIFOFF` writer - Clear clock changer FIFO full flag
pub type CCCFIFOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRCERRF` writer - Clear CRC error flag
pub type CCRCERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CECCERRF` writer - Clear ECC error flag
pub type CECCERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCECCERRF` writer - Clear corrected ECC error flag
pub type CCECCERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CIDERRF` writer - Clear data type ID error flag
pub type CIDERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSPKTERRF` writer - Clear short packet error flag
pub type CSPKTERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CWDERRF` writer - Clear watchdog error flag
pub type CWDERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSYNCERRF` writer - Clear invalid synchronization error flag
pub type CSYNCERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FCR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Clear line/byte counter 0 flag
    #[inline(always)]
    pub fn clb0f(&mut self) -> CLB0F_W<'_, FCR0rs> {
        CLB0F_W::new(self, 0)
    }
    ///Bit 1 - Clear line/byte counter 1 flag
    #[inline(always)]
    pub fn clb1f(&mut self) -> CLB1F_W<'_, FCR0rs> {
        CLB1F_W::new(self, 1)
    }
    ///Bit 2 - Clear line/byte counter 2 flag
    #[inline(always)]
    pub fn clb2f(&mut self) -> CLB2F_W<'_, FCR0rs> {
        CLB2F_W::new(self, 2)
    }
    ///Bit 3 - Clear line/byte counter 3 flag
    #[inline(always)]
    pub fn clb3f(&mut self) -> CLB3F_W<'_, FCR0rs> {
        CLB3F_W::new(self, 3)
    }
    ///Bit 4 - Clear timer 0 flag
    #[inline(always)]
    pub fn ctim0f(&mut self) -> CTIM0F_W<'_, FCR0rs> {
        CTIM0F_W::new(self, 4)
    }
    ///Bit 5 - Clear timer 1 flag
    #[inline(always)]
    pub fn ctim1f(&mut self) -> CTIM1F_W<'_, FCR0rs> {
        CTIM1F_W::new(self, 5)
    }
    ///Bit 6 - Clear timer 2 flag
    #[inline(always)]
    pub fn ctim2f(&mut self) -> CTIM2F_W<'_, FCR0rs> {
        CTIM2F_W::new(self, 6)
    }
    ///Bit 7 - Clear timer 3 flag
    #[inline(always)]
    pub fn ctim3f(&mut self) -> CTIM3F_W<'_, FCR0rs> {
        CTIM3F_W::new(self, 7)
    }
    ///Bit 8 - Clear SOF flag for virtual channel 0
    #[inline(always)]
    pub fn csof0f(&mut self) -> CSOF0F_W<'_, FCR0rs> {
        CSOF0F_W::new(self, 8)
    }
    ///Bit 9 - Clear SOF flag for virtual channel 1
    #[inline(always)]
    pub fn csof1f(&mut self) -> CSOF1F_W<'_, FCR0rs> {
        CSOF1F_W::new(self, 9)
    }
    ///Bit 10 - Clear SOF flag for virtual channel 2
    #[inline(always)]
    pub fn csof2f(&mut self) -> CSOF2F_W<'_, FCR0rs> {
        CSOF2F_W::new(self, 10)
    }
    ///Bit 11 - Clear SOF flag for virtual channel 3
    #[inline(always)]
    pub fn csof3f(&mut self) -> CSOF3F_W<'_, FCR0rs> {
        CSOF3F_W::new(self, 11)
    }
    ///Bit 12 - Clear EOF flag for virtual channel 0
    #[inline(always)]
    pub fn ceof0f(&mut self) -> CEOF0F_W<'_, FCR0rs> {
        CEOF0F_W::new(self, 12)
    }
    ///Bit 13 - Clear EOF flag for virtual channel 1
    #[inline(always)]
    pub fn ceof1f(&mut self) -> CEOF1F_W<'_, FCR0rs> {
        CEOF1F_W::new(self, 13)
    }
    ///Bit 14 - Clear EOF flag for virtual channel 2
    #[inline(always)]
    pub fn ceof2f(&mut self) -> CEOF2F_W<'_, FCR0rs> {
        CEOF2F_W::new(self, 14)
    }
    ///Bit 15 - Clear EOF flag for virtual channel 3
    #[inline(always)]
    pub fn ceof3f(&mut self) -> CEOF3F_W<'_, FCR0rs> {
        CEOF3F_W::new(self, 15)
    }
    ///Bit 16 - Clear short packet flag
    #[inline(always)]
    pub fn cspktf(&mut self) -> CSPKTF_W<'_, FCR0rs> {
        CSPKTF_W::new(self, 16)
    }
    ///Bit 21 - Clear clock changer FIFO full flag
    #[inline(always)]
    pub fn cccfifoff(&mut self) -> CCCFIFOFF_W<'_, FCR0rs> {
        CCCFIFOFF_W::new(self, 21)
    }
    ///Bit 24 - Clear CRC error flag
    #[inline(always)]
    pub fn ccrcerrf(&mut self) -> CCRCERRF_W<'_, FCR0rs> {
        CCRCERRF_W::new(self, 24)
    }
    ///Bit 25 - Clear ECC error flag
    #[inline(always)]
    pub fn ceccerrf(&mut self) -> CECCERRF_W<'_, FCR0rs> {
        CECCERRF_W::new(self, 25)
    }
    ///Bit 26 - Clear corrected ECC error flag
    #[inline(always)]
    pub fn cceccerrf(&mut self) -> CCECCERRF_W<'_, FCR0rs> {
        CCECCERRF_W::new(self, 26)
    }
    ///Bit 27 - Clear data type ID error flag
    #[inline(always)]
    pub fn ciderrf(&mut self) -> CIDERRF_W<'_, FCR0rs> {
        CIDERRF_W::new(self, 27)
    }
    ///Bit 28 - Clear short packet error flag
    #[inline(always)]
    pub fn cspkterrf(&mut self) -> CSPKTERRF_W<'_, FCR0rs> {
        CSPKTERRF_W::new(self, 28)
    }
    ///Bit 29 - Clear watchdog error flag
    #[inline(always)]
    pub fn cwderrf(&mut self) -> CWDERRF_W<'_, FCR0rs> {
        CWDERRF_W::new(self, 29)
    }
    ///Bit 30 - Clear invalid synchronization error flag
    #[inline(always)]
    pub fn csyncerrf(&mut self) -> CSYNCERRF_W<'_, FCR0rs> {
        CSYNCERRF_W::new(self, 30)
    }
}
/**CSI-2 Host flag clear register 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CSI:FCR0)*/
pub struct FCR0rs;
impl crate::RegisterSpec for FCR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fcr0::W`](W) writer structure
impl crate::Writable for FCR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FCR0 to value 0
impl crate::Resettable for FCR0rs {}
