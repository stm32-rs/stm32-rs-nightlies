#[doc = "Register `FMC_BTR2` reader"]
pub struct R(crate::R<FMC_BTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_BTR2` writer"]
pub struct W(crate::W<FMC_BTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_BTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FMC_BTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_BTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDSET` reader - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure81 to Figure93), used in SRAMs, ROMs and asynchronous NOR Flash: For each access mode address setup phase duration, please refer to the respective figure (refer to Figure81 to Figure93). Note: In synchronous accesses, this value is dont care. In Muxed mode or Mode D, the minimum value for ADDSET is 1."]
pub struct ADDSET_R(crate::FieldReader<u8, u8>);
impl ADDSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDSET` writer - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure81 to Figure93), used in SRAMs, ROMs and asynchronous NOR Flash: For each access mode address setup phase duration, please refer to the respective figure (refer to Figure81 to Figure93). Note: In synchronous accesses, this value is dont care. In Muxed mode or Mode D, the minimum value for ADDSET is 1."]
pub struct ADDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `ADDHLD` reader - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in mode D or multiplexed accesses: For each access mode address-hold phase duration, please refer to the respective figure (Figure81 to Figure93). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
pub struct ADDHLD_R(crate::FieldReader<u8, u8>);
impl ADDHLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDHLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDHLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDHLD` writer - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in mode D or multiplexed accesses: For each access mode address-hold phase duration, please refer to the respective figure (Figure81 to Figure93). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
pub struct ADDHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDHLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DATAST` reader - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous accesses: For each memory type and access mode data-phase duration, please refer to the respective figure (Figure81 to Figure93). Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 KCK_FMC clock cycles. Note: In synchronous accesses, this value is dont care."]
pub struct DATAST_R(crate::FieldReader<u8, u8>);
impl DATAST_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAST` writer - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous accesses: For each memory type and access mode data-phase duration, please refer to the respective figure (Figure81 to Figure93). Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 KCK_FMC clock cycles. Note: In synchronous accesses, this value is dont care."]
pub struct DATAST_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BUSTURN` reader - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read or read-to write transaction. The programmed bus turnaround delay is inserted between an asynchronous read (in muxed or mode D) or write transaction and any other asynchronous /synchronous read/write from/to a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed mode and mode D. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for modes muxed and D. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except in modes muxed and D mode. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or a different one in case of a read operation. Two consecutive synchronous read operations (in Burst or Single mode) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write access (in Burst or Single mode) and a synchronous read from the same or a different bank. The bus turnaround delay allows to match the minimum time between consecutive transactions (tEHEL from NEx high to NEx low) and the maximum time required by the memory to free the data bus after a read access (tEHQZ): (BUSTRUN + 1) KCK_FMC period &#8805; tEHELmin and (BUSTRUN + 2)KCK_FMC period &#8805; tEHQZmax if EXTMOD = 0 (BUSTRUN + 2)KCK_FMC period &#8805; max (tEHELmin, tEHQZmax) if EXTMOD = 126. ..."]
pub struct BUSTURN_R(crate::FieldReader<u8, u8>);
impl BUSTURN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BUSTURN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSTURN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSTURN` writer - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read or read-to write transaction. The programmed bus turnaround delay is inserted between an asynchronous read (in muxed or mode D) or write transaction and any other asynchronous /synchronous read/write from/to a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed mode and mode D. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for modes muxed and D. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except in modes muxed and D mode. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or a different one in case of a read operation. Two consecutive synchronous read operations (in Burst or Single mode) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write access (in Burst or Single mode) and a synchronous read from the same or a different bank. The bus turnaround delay allows to match the minimum time between consecutive transactions (tEHEL from NEx high to NEx low) and the maximum time required by the memory to free the data bus after a read access (tEHQZ): (BUSTRUN + 1) KCK_FMC period &#8805; tEHELmin and (BUSTRUN + 2)KCK_FMC period &#8805; tEHQZmax if EXTMOD = 0 (BUSTRUN + 2)KCK_FMC period &#8805; max (tEHELmin, tEHQZmax) if EXTMOD = 126. ..."]
pub struct BUSTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSTURN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `CLKDIV` reader - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of KCK_FMC cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section20.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
pub struct CLKDIV_R(crate::FieldReader<u8, u8>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of KCK_FMC cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section20.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `DATLAT` reader - Data latency for synchronous memory For synchronous access with read write burst mode enabled these bits define the number of memory clock cycles"]
pub struct DATLAT_R(crate::FieldReader<u8, u8>);
impl DATLAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATLAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATLAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATLAT` writer - Data latency for synchronous memory For synchronous access with read write burst mode enabled these bits define the number of memory clock cycles"]
pub struct DATLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `ACCMOD` reader - Access mode These bits specify the asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub struct ACCMOD_R(crate::FieldReader<u8, u8>);
impl ACCMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACCMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCMOD` writer - Access mode These bits specify the asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
pub struct ACCMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `DATAHLD` reader - DATAHLD"]
pub struct DATAHLD_R(crate::FieldReader<u8, u8>);
impl DATAHLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAHLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAHLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAHLD` writer - DATAHLD"]
pub struct DATAHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAHLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure81 to Figure93), used in SRAMs, ROMs and asynchronous NOR Flash: For each access mode address setup phase duration, please refer to the respective figure (refer to Figure81 to Figure93). Note: In synchronous accesses, this value is dont care. In Muxed mode or Mode D, the minimum value for ADDSET is 1."]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in mode D or multiplexed accesses: For each access mode address-hold phase duration, please refer to the respective figure (Figure81 to Figure93). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous accesses: For each memory type and access mode data-phase duration, please refer to the respective figure (Figure81 to Figure93). Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 KCK_FMC clock cycles. Note: In synchronous accesses, this value is dont care."]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read or read-to write transaction. The programmed bus turnaround delay is inserted between an asynchronous read (in muxed or mode D) or write transaction and any other asynchronous /synchronous read/write from/to a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed mode and mode D. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for modes muxed and D. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except in modes muxed and D mode. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or a different one in case of a read operation. Two consecutive synchronous read operations (in Burst or Single mode) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write access (in Burst or Single mode) and a synchronous read from the same or a different bank. The bus turnaround delay allows to match the minimum time between consecutive transactions (tEHEL from NEx high to NEx low) and the maximum time required by the memory to free the data bus after a read access (tEHQZ): (BUSTRUN + 1) KCK_FMC period &#8805; tEHELmin and (BUSTRUN + 2)KCK_FMC period &#8805; tEHQZmax if EXTMOD = 0 (BUSTRUN + 2)KCK_FMC period &#8805; max (tEHELmin, tEHQZmax) if EXTMOD = 126. ..."]
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of KCK_FMC cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section20.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Data latency for synchronous memory For synchronous access with read write burst mode enabled these bits define the number of memory clock cycles"]
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Access mode These bits specify the asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - DATAHLD"]
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup phase duration These bits are written by software to define the duration of the address setup phase (refer to Figure81 to Figure93), used in SRAMs, ROMs and asynchronous NOR Flash: For each access mode address setup phase duration, please refer to the respective figure (refer to Figure81 to Figure93). Note: In synchronous accesses, this value is dont care. In Muxed mode or Mode D, the minimum value for ADDSET is 1."]
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W {
        ADDSET_W { w: self }
    }
    #[doc = "Bits 4:7 - Address-hold phase duration These bits are written by software to define the duration of the address hold phase (refer to Figure81 to Figure93), used in mode D or multiplexed accesses: For each access mode address-hold phase duration, please refer to the respective figure (Figure81 to Figure93). Note: In synchronous accesses, this value is not used, the address hold phase is always 1 memory clock period duration."]
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W {
        ADDHLD_W { w: self }
    }
    #[doc = "Bits 8:15 - Data-phase duration These bits are written by software to define the duration of the data phase (refer to Figure81 to Figure93), used in asynchronous accesses: For each memory type and access mode data-phase duration, please refer to the respective figure (Figure81 to Figure93). Example: Mode1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 KCK_FMC clock cycles. Note: In synchronous accesses, this value is dont care."]
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W {
        DATAST_W { w: self }
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration These bits are written by software to add a delay at the end of a write-to-read or read-to write transaction. The programmed bus turnaround delay is inserted between an asynchronous read (in muxed or mode D) or write transaction and any other asynchronous /synchronous read/write from/to a static bank. If a read operation is performed, the bank can be the same or a different one, whereas it must be different in case of write operation to the bank, except in muxed mode or mode D. In some cases, whatever the programmed BUSTRUN values, the bus turnaround delay is fixed as follows: The bus turnaround delay is not inserted between two consecutive asynchronous write transfers to the same static memory bank except in muxed mode and mode D. There is a bus turnaround delay of 1 FMC clock cycle between: Two consecutive asynchronous read transfers to the same static memory bank except for modes muxed and D. An asynchronous read to an asynchronous or synchronous write to any static bank or dynamic bank except in modes muxed and D mode. There is a bus turnaround delay of 2 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to the same bank. A synchronous write (burst or single) access and an asynchronous write or read transfer to or from static memory bank (the bank can be the same or a different one in case of a read operation. Two consecutive synchronous read operations (in Burst or Single mode) followed by any synchronous/asynchronous read or write from/to another static memory bank. There is a bus turnaround delay of 3 FMC clock cycle between: Two consecutive synchronous write operations (in Burst or Single mode) to different static banks. A synchronous write access (in Burst or Single mode) and a synchronous read from the same or a different bank. The bus turnaround delay allows to match the minimum time between consecutive transactions (tEHEL from NEx high to NEx low) and the maximum time required by the memory to free the data bus after a read access (tEHQZ): (BUSTRUN + 1) KCK_FMC period &#8805; tEHELmin and (BUSTRUN + 2)KCK_FMC period &#8805; tEHQZmax if EXTMOD = 0 (BUSTRUN + 2)KCK_FMC period &#8805; max (tEHELmin, tEHQZmax) if EXTMOD = 126. ..."]
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W {
        BUSTURN_W { w: self }
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FMC_CLK signal) These bits define the period of FMC_CLK clock output signal, expressed in number of KCK_FMC cycles: In asynchronous NOR Flash, SRAM or PSRAM accesses, this value is dont care. Note: Refer to Section20.6.5: Synchronous transactions for FMC_CLK divider ratio formula)"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bits 24:27 - Data latency for synchronous memory For synchronous access with read write burst mode enabled these bits define the number of memory clock cycles"]
    #[inline(always)]
    pub fn datlat(&mut self) -> DATLAT_W {
        DATLAT_W { w: self }
    }
    #[doc = "Bits 28:29 - Access mode These bits specify the asynchronous access modes as shown in the timing diagrams. They are taken into account only when the EXTMOD bit in the FMC_BCRx register is 1."]
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W {
        ACCMOD_W { w: self }
    }
    #[doc = "Bits 30:31 - DATAHLD"]
    #[inline(always)]
    pub fn datahld(&mut self) -> DATAHLD_W {
        DATAHLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC_BTR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_btr2](index.html) module"]
pub struct FMC_BTR2_SPEC;
impl crate::RegisterSpec for FMC_BTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_btr2::R](R) reader structure"]
impl crate::Readable for FMC_BTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_btr2::W](W) writer structure"]
impl crate::Writable for FMC_BTR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_BTR2 to value 0x0fff_ffff"]
impl crate::Resettable for FMC_BTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_ffff
    }
}
