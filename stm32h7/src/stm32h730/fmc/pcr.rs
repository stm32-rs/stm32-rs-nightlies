///Register `PCR` reader
pub type R = crate::R<PCRrs>;
///Register `PCR` writer
pub type W = crate::W<PCRrs>;
///Field `PWAITEN` reader - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:
pub type PWAITEN_R = crate::BitReader;
///Field `PWAITEN` writer - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PBKEN` reader - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus
pub type PBKEN_R = crate::BitReader;
///Field `PBKEN` writer - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWID` reader - Data bus width. These bits define the external memory device width.
pub type PWID_R = crate::FieldReader;
///Field `PWID` writer - Data bus width. These bits define the external memory device width.
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ECCEN` reader - ECC computation logic enable bit
pub type ECCEN_R = crate::BitReader;
///Field `ECCEN` writer - ECC computation logic enable bit
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCLR` reader - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TCLR_R = crate::FieldReader;
///Field `TCLR` writer - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TAR` reader - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR_R = crate::FieldReader;
///Field `TAR` writer - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ECCPS` reader - ECC page size. These bits define the page size for the extended ECC:
pub type ECCPS_R = crate::FieldReader;
///Field `ECCPS` writer - ECC page size. These bits define the page size for the extended ECC:
pub type ECCPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 1 - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus
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
    ///Bits 9:12 - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:16 - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
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
            .field("tar", &self.tar())
            .field("eccps", &self.eccps())
            .finish()
    }
}
impl W {
    ///Bit 1 - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W<PCRrs> {
        PWAITEN_W::new(self, 1)
    }
    ///Bit 2 - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W<PCRrs> {
        PBKEN_W::new(self, 2)
    }
    ///Bits 4:5 - Data bus width. These bits define the external memory device width.
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W<PCRrs> {
        PWID_W::new(self, 4)
    }
    ///Bit 6 - ECC computation logic enable bit
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W<PCRrs> {
        ECCEN_W::new(self, 6)
    }
    ///Bits 9:12 - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W<PCRrs> {
        TCLR_W::new(self, 9)
    }
    ///Bits 13:16 - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space.
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W<PCRrs> {
        TAR_W::new(self, 13)
    }
    ///Bits 17:19 - ECC page size. These bits define the page size for the extended ECC:
    #[inline(always)]
    pub fn eccps(&mut self) -> ECCPS_W<PCRrs> {
        ECCPS_W::new(self, 17)
    }
}
/**NAND Flash control registers

You can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#FMC:PCR)*/
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
///`read()` method returns [`pcr::R`](R) reader structure
impl crate::Readable for PCRrs {}
///`write(|w| ..)` method takes [`pcr::W`](W) writer structure
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCR to value 0x18
impl crate::Resettable for PCRrs {
    const RESET_VALUE: u32 = 0x18;
}
