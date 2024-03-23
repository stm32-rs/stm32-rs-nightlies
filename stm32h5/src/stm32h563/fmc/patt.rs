#[doc = "Register `PATT` reader"]
pub type R = crate::R<PATTrs>;
#[doc = "Register `PATT` writer"]
pub type W = crate::W<PATTrs>;
#[doc = "Field `ATTSET` reader - Attribute memory setup time Defines the number of HCLK (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:"]
pub type ATTSET_R = crate::FieldReader;
#[doc = "Field `ATTSET` writer - Attribute memory setup time Defines the number of HCLK (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:"]
pub type ATTSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTWAIT` reader - Attribute memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:"]
pub type ATTWAIT_R = crate::FieldReader;
#[doc = "Field `ATTWAIT` writer - Attribute memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:"]
pub type ATTWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHOLD` reader - Attribute memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write access) after the command deassertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:"]
pub type ATTHOLD_R = crate::FieldReader;
#[doc = "Field `ATTHOLD` writer - Attribute memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write access) after the command deassertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:"]
pub type ATTHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHIZ` reader - Attribute memory data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:"]
pub type ATTHIZ_R = crate::FieldReader;
#[doc = "Field `ATTHIZ` writer - Attribute memory data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:"]
pub type ATTHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Attribute memory setup time Defines the number of HCLK (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:"]
    #[inline(always)]
    pub fn attset(&self) -> ATTSET_R {
        ATTSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:"]
    #[inline(always)]
    pub fn attwait(&self) -> ATTWAIT_R {
        ATTWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write access) after the command deassertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:"]
    #[inline(always)]
    pub fn atthold(&self) -> ATTHOLD_R {
        ATTHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Attribute memory data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:"]
    #[inline(always)]
    pub fn atthiz(&self) -> ATTHIZ_R {
        ATTHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Attribute memory setup time Defines the number of HCLK (+1) clock cycles to set up address before the command assertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:"]
    #[inline(always)]
    #[must_use]
    pub fn attset(&mut self) -> ATTSET_W<PATTrs> {
        ATTSET_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Attribute memory wait time Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket x. The duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the end of the programmed value of HCLK:"]
    #[inline(always)]
    #[must_use]
    pub fn attwait(&mut self) -> ATTWAIT_W<PATTrs> {
        ATTWAIT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Attribute memory hold time Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for read access during which the address is held (and data for write access) after the command deassertion (NWE, NOE), for NAND Flash read or write access to attribute memory space on socket:"]
    #[inline(always)]
    #[must_use]
    pub fn atthold(&mut self) -> ATTHOLD_W<PATTrs> {
        ATTHOLD_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Attribute memory data bus Hi-Z time Defines the number of HCLK clock cycles during which the data bus is kept in Hi-Z after the start of a NAND Flash write access to attribute memory space on socket. Only valid for writ transaction:"]
    #[inline(always)]
    #[must_use]
    pub fn atthiz(&mut self) -> ATTHIZ_W<PATTrs> {
        ATTHIZ_W::new(self, 24)
    }
}
#[doc = "Attribute memory space timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PATTrs;
impl crate::RegisterSpec for PATTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`patt::R`](R) reader structure"]
impl crate::Readable for PATTrs {}
#[doc = "`write(|w| ..)` method takes [`patt::W`](W) writer structure"]
impl crate::Writable for PATTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PATT to value 0xfcfc_fcfc"]
impl crate::Resettable for PATTrs {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
