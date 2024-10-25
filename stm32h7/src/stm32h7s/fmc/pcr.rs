///Register `PCR` reader
pub type R = crate::R<PCRrs>;
///Register `PCR` writer
pub type W = crate::W<PCRrs>;
///Field `PWAITEN` reader - Wait feature enable bit. This bit enables the Wait feature for the NAND flash memory bank:
pub type PWAITEN_R = crate::BitReader;
///Field `PWAITEN` writer - Wait feature enable bit. This bit enables the Wait feature for the NAND flash memory bank:
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PBKEN` reader - NAND flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus
pub type PBKEN_R = crate::BitReader;
///Field `PBKEN` writer - NAND flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWID` reader - Data bus width. These bits define the external memory device width.
pub type PWID_R = crate::FieldReader;
///Field `PWID` writer - Data bus width. These bits define the external memory device width.
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ECCEN` reader - ECC computation logic enable bit
pub type ECCEN_R = crate::BitReader;
///Field `ECCEN` writer - ECC computation logic enable bit
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLR` reader - CLE to RE delay. These bits set time from CLE low to RE low in number of fmc_ker_ck clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the fmc_ker_ck clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TCLR_R = crate::FieldReader;
///Field `TCLR` writer - CLE to RE delay. These bits set time from CLE low to RE low in number of fmc_ker_ck clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the fmc_ker_ck clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TAR0` reader - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR0_R = crate::BitReader;
///Field `TAR0` writer - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAR1` reader - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR1_R = crate::BitReader;
///Field `TAR1` writer - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAR2` reader - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR2_R = crate::BitReader;
///Field `TAR2` writer - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAR3` reader - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR3_R = crate::BitReader;
///Field `TAR3` writer - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCPS` reader - ECC page size. These bits define the page size for the extended ECC:
pub type ECCPS_R = crate::FieldReader;
///Field `ECCPS` writer - ECC page size. These bits define the page size for the extended ECC:
pub type ECCPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 1 - Wait feature enable bit. This bit enables the Wait feature for the NAND flash memory bank:
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NAND flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:5 - Data bus width. These bits define the external memory device width.
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - ECC computation logic enable bit
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 9:12 - CLE to RE delay. These bits set time from CLE low to RE low in number of fmc_ker_ck clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the fmc_ker_ck clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bit 13 - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    pub fn tar0(&self) -> TAR0_R {
        TAR0_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    pub fn tar1(&self) -> TAR1_R {
        TAR1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    pub fn tar2(&self) -> TAR2_R {
        TAR2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    pub fn tar3(&self) -> TAR3_R {
        TAR3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - ECC page size. These bits define the page size for the extended ECC:
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR")
            .field("pwaiten", &self.pwaiten())
            .field("pbken", &self.pbken())
            .field("pwid", &self.pwid())
            .field("eccen", &self.eccen())
            .field("tclr", &self.tclr())
            .field("tar0", &self.tar0())
            .field("tar1", &self.tar1())
            .field("tar2", &self.tar2())
            .field("tar3", &self.tar3())
            .field("eccps", &self.eccps())
            .finish()
    }
}
impl W {
    ///Bit 1 - Wait feature enable bit. This bit enables the Wait feature for the NAND flash memory bank:
    #[inline(always)]
    #[must_use]
    pub fn pwaiten(&mut self) -> PWAITEN_W<PCRrs> {
        PWAITEN_W::new(self, 1)
    }
    ///Bit 2 - NAND flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus
    #[inline(always)]
    #[must_use]
    pub fn pbken(&mut self) -> PBKEN_W<PCRrs> {
        PBKEN_W::new(self, 2)
    }
    ///Bits 4:5 - Data bus width. These bits define the external memory device width.
    #[inline(always)]
    #[must_use]
    pub fn pwid(&mut self) -> PWID_W<PCRrs> {
        PWID_W::new(self, 4)
    }
    ///Bit 6 - ECC computation logic enable bit
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<PCRrs> {
        ECCEN_W::new(self, 6)
    }
    ///Bits 9:12 - CLE to RE delay. These bits set time from CLE low to RE low in number of fmc_ker_ck clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the fmc_ker_ck clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<PCRrs> {
        TCLR_W::new(self, 9)
    }
    ///Bit 13 - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    #[must_use]
    pub fn tar0(&mut self) -> TAR0_W<PCRrs> {
        TAR0_W::new(self, 13)
    }
    ///Bit 14 - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    #[must_use]
    pub fn tar1(&mut self) -> TAR1_W<PCRrs> {
        TAR1_W::new(self, 14)
    }
    ///Bit 15 - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    #[must_use]
    pub fn tar2(&mut self) -> TAR2_W<PCRrs> {
        TAR2_W::new(self, 15)
    }
    ///Bit 16 - ALE to RE delay. These bits set time from ALE low to RE low in number of fmc_ker_ck clock cycles. Time is: t_ar = (TAR + SET + 2) t&lt;sub>fmc_ker_ck&lt;/sub> where t&lt;sub>fmc_ker_ck&lt;/sub> is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    #[must_use]
    pub fn tar3(&mut self) -> TAR3_W<PCRrs> {
        TAR3_W::new(self, 16)
    }
    ///Bits 17:19 - ECC page size. These bits define the page size for the extended ECC:
    #[inline(always)]
    #[must_use]
    pub fn eccps(&mut self) -> ECCPS_W<PCRrs> {
        ECCPS_W::new(self, 17)
    }
}
/**NAND flash control registers

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FMC:PCR)*/
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
///`read()` method returns [`pcr::R`](R) reader structure
impl crate::Readable for PCRrs {}
///`write(|w| ..)` method takes [`pcr::W`](W) writer structure
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PCR to value 0x18
impl crate::Resettable for PCRrs {
    const RESET_VALUE: u32 = 0x18;
}
