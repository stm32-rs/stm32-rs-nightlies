#[doc = "Register `PMEM` reader"]
pub type R = crate::R<PMEMrs>;
#[doc = "Register `PMEM` writer"]
pub type W = crate::W<PMEMrs>;
#[doc = "Field `MEMSET` reader - Common memory x setup time Defines the number of HCLK (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:"]
pub type MEMSET_R = crate::FieldReader;
#[doc = "Field `MEMSET` writer - Common memory x setup time Defines the number of HCLK (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:"]
pub type MEMSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMWAIT` reader - Common memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space on socket. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:"]
pub type MEMWAIT_R = crate::FieldReader;
#[doc = "Field `MEMWAIT` writer - Common memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space on socket. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:"]
pub type MEMWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHOLD` reader - Common memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write accesses) after the command is deasserted (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:"]
pub type MEMHOLD_R = crate::FieldReader;
#[doc = "Field `MEMHOLD` writer - Common memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write accesses) after the command is deasserted (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:"]
pub type MEMHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHIZ` reader - Common memory x data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space on socket. This is only valid for write transactions:"]
pub type MEMHIZ_R = crate::FieldReader;
#[doc = "Field `MEMHIZ` writer - Common memory x data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space on socket. This is only valid for write transactions:"]
pub type MEMHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Common memory x setup time Defines the number of HCLK (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Common memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space on socket. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:"]
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Common memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write accesses) after the command is deasserted (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:"]
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Common memory x data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space on socket. This is only valid for write transactions:"]
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Common memory x setup time Defines the number of HCLK (+1) clock cycles to set up the address before the command assertion (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:"]
    #[inline(always)]
    #[must_use]
    pub fn memset(&mut self) -> MEMSET_W<PMEMrs> {
        MEMSET_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Common memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to common memory space on socket. The duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:"]
    #[inline(always)]
    #[must_use]
    pub fn memwait(&mut self) -> MEMWAIT_W<PMEMrs> {
        MEMWAIT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Common memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write accesses) after the command is deasserted (NWE, NOE), for NAND Flash read or write access to common memory space on socket x:"]
    #[inline(always)]
    #[must_use]
    pub fn memhold(&mut self) -> MEMHOLD_W<PMEMrs> {
        MEMHOLD_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Common memory x data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept Hi-Z after the start of a NAND Flash write access to common memory space on socket. This is only valid for write transactions:"]
    #[inline(always)]
    #[must_use]
    pub fn memhiz(&mut self) -> MEMHIZ_W<PMEMrs> {
        MEMHIZ_W::new(self, 24)
    }
}
#[doc = "Common memory space timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMEMrs;
impl crate::RegisterSpec for PMEMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmem::R`](R) reader structure"]
impl crate::Readable for PMEMrs {}
#[doc = "`write(|w| ..)` method takes [`pmem::W`](W) writer structure"]
impl crate::Writable for PMEMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMEM to value 0xfcfc_fcfc"]
impl crate::Resettable for PMEMrs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
